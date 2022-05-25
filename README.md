
# WIP
This project was my intro to learning rust and is still early on in its development

Planned Features:

     - post
        - title
        - author
        - date (published) (edited) 
        - body
            - body text 
            - pictures and gifs
            - links
            - video
            - unicode
        - ID
        - tags
        - catagory
        - visibility

    - post storage
        - find post
        - store post
        - load post
        - resource saving (gifs, pdfs, pics)
        - delete post (hide and perm delete)
    
    - recouce
        - name
        - ID
        - Type


    - post editor
        - web app
        - fill out all the feilds in a post
        - upoad resources

    - post veiwer
        - show latest blogs
        - catagrized blogs
        - most popular
        - search blogs



## Architecture

- postd 
    - Runs on the server
    - Post daemon
    - Jobs
        - host the post database
        - host the website
            - see blogapp

- blogapp
    - Webapp, runs on the the browser
    - Communicates to the postd backend


