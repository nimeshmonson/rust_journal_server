# Journal Server - Rust
This is an implementation of a server that can be used to maintain a journal. The journal takes a _phrase_ and appropriately organizes it into files for each day a journal entry is made. For a day with multiple entries, the journal appends the phrase to that day's journal with a timestamp. 

This project uses actix-web and file i/o to accomplish this. The server is also set to host on ```127.0.0.1:8080```

To have the server up and running, run ```cargo build``` followed by ```cargo run```. Make sure you have rust installed before doing so.

The server currently has four endpoints:
* GET / : index that lists all the endpoints
    * Example : ```curl -X GET 127.0.0.1```
    
* GET /ping : endpoint to check if the server is running
    * Example : ```curl -X GET 127.0.0.1/ping```
    
* POST /journal : endpoint to submit a phrase to be stored
    * Example : ```curl -X POST '127.0.0.1:8080/journal?phrase=this%20is%20a%20thought'```-> posts a phrase _this is a thought_ to today's date
    
* GET /journal : endpoint to retrieve journal entries for any day
    * Example : ```curl -X GET '127.0.0.1:8080/journal?date=17&month=9&year=2020'``` -> returns the journal from Sept 17th, 2020

