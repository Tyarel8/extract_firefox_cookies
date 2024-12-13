# extract_firefox_cookies

Extract cookies from firefox sqlite db and session jsonlz4.

```
Options:
  -p, --profile <PROFILE>
          Use non-default firefox profile

  -d, --domain <DOMAIN>
          Filter the cookies by domain (matches `<DOMAIN>` and `.<DOMAIN>`)

  -o, --output-format <OUTPUT_FORMAT>
          Cookie output format
          
          [default: javascript]

          Possible values:
          - javascript: Default javascript format
          - netscape:   Netscape format, compatible with curl & wget
          - json:       Json format

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
