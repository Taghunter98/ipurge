# ipurge

Software to purge unsued image directories for Whondo, to be run as a cronjob.

## Examples and Usage

Create a .env file with the endpoint and folder destination for purging.

```plaintext
URL='https://myendpoint'
DIR_PATH='/home/files'
```

Then build and run the apllication passing the cronjob time as the argument.

## Examples
Run the cronjob once a day at midnight
```bash
cargo run '0 0 * * * *' 'https://website.com/endpoint' '/home/user/purgedirectoty'
```

Run the cronjob every five seconds
```bash
cargo run '0/5 * * * * *' 'https://website.com/endpoint' '/home/user/purgedirectoty'
```

## License

Copyright (C) Josh Bassett, www.whondo.com. All rights reserved.
