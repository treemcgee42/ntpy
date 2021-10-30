# Design for ntpy

## Database

### Type
sqlite

### Schema
This was heavily inspired by the mediawiki schema, found at https://upload.wikimedia.org/wikipedia/commons/9/94/MediaWiki_1.28.0_database_schema.svg.

There are two databases: `page` and `text`. 

`page` holds the information about the page but not the text itself, and `text` holds just the 
text of a page (currently this is just the most recent version, eventually could have a different 
entry corresponding to each version).

### What is stored
At this point, we only store the latest version of each page. However, I am aware that wikis 
typically store every revision. That is reflected in the database schema: it should be general
enough to eventually support this feature in the future.
