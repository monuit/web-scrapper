#!/usr/bin/perl
use JSON;
use File::Slurp;
use Data::UUID;
use DateTime;
use AnyEvent;
use AnyEvent::Socket;
use AnyEvent::Handle;
use Digest::SHA qw(sha256_hex);

my $cv = AnyEvent->condvar;

tcp_server '127.0.0.1', 8080, sub {
    my ($fh) = @_;
    my $handle;
    $handle = new AnyEvent::Handle(fh => $fh);
    $handle->on_read(
        sub {
            my $data = $handle->rbuf;
            $handle->rbuf = "";
            
            # Decrypt using SHA-256
            my $decrypted_data = sha256_hex($data);
            
            # Process decrypted data
            # Here you can call the main() subroutine to proceed with the scraping functionalities
            main($decrypted_data);
        }
    );
};

$cv->recv;

# Function to parse HTML text and extract links, image sources, and paragraph texts
sub parse_text {
    my $text = shift;
    my @links = $text =~ m{href=["'](https?://.+?)["']}g;
    my @images = $text =~ m{src=["'](https?://.+?)["']}g;
    my @paragraphs = $text =~ m{<p>(.+?)</p>}g;

    return {links => \@links, images => \@images, paragraphs => \@paragraphs};
}

# Function to save reply log
sub save_reply_log {
    my ($request, $url, $identifier) = @_;
    my $dt = DateTime->now(time_zone => 'local');
    my $timestamp = $dt->datetime();
    my $log_data = {
        request => $request,
        url => $url,
        timestamp => $timestamp,
        identifier => $identifier
    };
    my $file_name = "replies/" . Data::UUID->new()->create_str() . ".json";
    write_file($file_name, encode_json $log_data);
}

# Updated main function
sub main {
    my $decrypted_data = shift;  # The decrypted data from the socket

    print "Enter HTML content: ";
    my $text_content = <STDIN>;
    chomp $text_content;
    
    print "Enter URL: ";
    my $url = <STDIN>;
    chomp $url;

    print "Enter HTTP request method (e.g., GET, POST): ";
    my $request = <STDIN>;
    chomp $request;

    my $identifier = Data::UUID->new()->create_str();  # Generate a UUID for this operation

    my $parsed_data = parse_text($text_content);
    print "Extracted Links: ", join(", ", @{$parsed_data->{links}}), "\n";
    print "Extracted Image Sources: ", join(", ", @{$parsed_data->{images}}), "\n";
    print "Extracted Paragraphs: ", join(", ", @{$parsed_data->{paragraphs}}), "\n";

    save_reply_log($request, $url, $identifier);
}

# Call the main subroutine
main();
