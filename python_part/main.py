import pandas as pd
import json
import uuid
import os
import asyncio
import hashlib
from datetime import datetime
from pyarrow import parquet as pq
from pyarrow import Table
from cryptography.fernet import Fernet

# Generate a key for encryption and decryption
key = Fernet.generate_key()
cipher_suite = Fernet(key)


def encrypt_data(data):
    encrypted_data = cipher_suite.encrypt(data.encode())
    return encrypted_data

def decrypt_data(encrypted_data):
    decrypted_data = cipher_suite.decrypt(encrypted_data).decode()
    return decrypted_data

async def handle_client(reader, writer):
    retry_count = 0
    max_retries = 3
    retry_delay = 2  # 2 seconds

    while retry_count < max_retries:
        try:
            data = await reader.read(100)
            if not data:
                raise ValueError("Received no data or connection was closed by the client.")

            # Decrypt using SHA-256
            hasher = hashlib.sha256()
            hasher.update(data)
            decrypted_data = hasher.digest()

            # Further decrypt the data if it was encrypted before being sent
            final_data = decrypt_data(decrypted_data.decode())

            # Process decrypted data
            main(final_data)
            break  # Exit the loop if everything is successful

        except Exception as e:
            print(f"An error occurred: {e}")
            retry_count += 1
            if retry_count < max_retries:
                print(f"Retrying... Attempt {retry_count}/{max_retries}")
                await asyncio.sleep(retry_delay)  # Delay before retrying
            else:
                print("Max retries reached. Exiting...")

async def socket_server():
    server = await asyncio.start_server(handle_client, '127.0.0.1', 8080)
    await server.serve_forever()

def save_request_log(request, url):
    timestamp = datetime.now().isoformat()
    log_data = {
        "request": request,
        "url": url,
        "timestamp": timestamp
    }
    file_name = f"requests/{uuid.uuid4()}.json"
    with open(file_name, 'w') as f:
        json.dump(log_data, f)

def save_temp_data(data, identifier):
    file_name = f"temp_data/{identifier}.txt"
    with open(file_name, 'w') as f:
        f.write(data)

def save_to_parquet(data, request_id, timestamp):
    random_digits = uuid.uuid4().int & (1<<10) - 1  # Generate random 10-bit integer
    file_name = f"data/{request_id}_{random_digits}_{timestamp}.parquet"

    # Convert the data to DataFrame and then to a Parquet table
    df = pd.DataFrame([data])
    table = Table.from_pandas(df)
    
    # Save the table to a Parquet file
    pq.write_table(table, file_name)

def main(decrypted_data):
    url = "http://example.com"  # Replace with the URL you want to scrape
    request_id = uuid.uuid4()  # Generate a UUID for this operation
    timestamp = datetime.now().isoformat()

    # Save the request log
    save_request_log("GET", url)
    
    # Save the data temporarily
    save_temp_data(decrypted_data, str(request_id))

    # Save to Parquet
    save_to_parquet(decrypted_data, str(request_id), timestamp)

if __name__ == "__main__":
    asyncio.run(socket_server())
