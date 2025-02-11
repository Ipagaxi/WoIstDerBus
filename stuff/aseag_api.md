## Requesting route from a -> b
- To request that you want a connection now you just leave out outTime and outDate
![diff now and spec. time](diff_now_time.png)
- To specify if arrival time or departure time matters, set outFrwd to false/true
![diff arrival time or dep time](diff_arr_dep.png)
- In ever request is an id counter that increases with each request. I dont know if that is somehow relevant. It is reseted when I reopen the browser, which is likely to happen because my browser deletes all data on close
- The request id changes with each session, but I dont think that it matters
![diff request id](diff_req_id.png)
- When the arrival location changes only the accorging json section changes
![arrival location changes](arrival_loc_change.png)
- When departure and arrival location switches, their ids and data are just switched in the json section as well and only the prefix "arr/dep" changes
![arrival dep loc switch](arr_dep_loc_switch.png)
![arrival dep data changes](arr_dep_data_changes.png)