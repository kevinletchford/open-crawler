# Open-Crawl

Open-Crawl is a fast, light-weight, open source crawler built in Rust. 

## Why another crawler?

There are many crawlers that already exist in some form. But they are normally tied to large corporate companies who want to control what or how you crawl the web.


## Data tables

- Project
- Audit
- Node
- Global Settings

### Project
Id
Name
BaseURL
CreationDate
LastUpdated
Analytics - GA,


### Audit
Id
Name
CreationDate
LastUpdated
CompletionDate
Status - Crawling, Processing, Paused, Completed
Log

### Node
Id
Url
Type - Page, Image, Video, js, css, pdf, 
Status

### Page
Id
PageScore
RawHtml
Content
Canonical
Index, - onPage, robots.txt, response code
Language, check if there are any references


## Plugin Architecture 
Plugins need to live in the following states:
- pre audit
    - has access to the audit configuration
- pre node capture
    - just before we capture the node, run code
- node renderer
    - configure the renderer and what it does when its capturing nodes
- post node capture
    - after we have captured the node data what do we do with it,
- post audit
    - After the audit has finished crawling 
- post processing
    - Process everything it the audit once its completed.

