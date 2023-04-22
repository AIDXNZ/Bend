# Bend 

This repo is for reconciling Sets of Hashes that represent files. It is not a Data transfer protocol. It is a POC to test and build.

### Initialization 
The first Node of the DB generates a keypair and creates UCAN proof for the DB. It also can generate a preshaed key
to use for acceess, otherwise read access is allowed by deafult. And write access needs to have a proof of delegation. 

### Authenticated Ops using UCANs
Using UCANs

### Rejoining a cluster after being down
If the set is currently unknown a peer will send the DB hash name with a UCAN and should recieve the full set.
It the set is partially known a peer will use a form of Range Based Set Reconciliation 


Ex. Psuedo for join

DB Name = 32 bit hash 
// All new items to the list are appended to the end, and deleted items are by index.
v1 = [1,2,3,4,5,6,7,8];
v2 = [1,2,5,6,7,8,9];


1. Alice starts a sync with Bob v2 by sending the len of her set and the last entry appended.
   This tells bob where the last time ALice was last appended and also how many deletions Alice is behind. 
2. Bob sends the rest of the set starting at Alices last position. The total num of deletions is n is v1 - v2 + len of sent items
    bob also sends the last n entries where n is (v1 - v2) + ( v1 - ( v2 - len(v2)))
3. Alice then Deletes the elements by applying the operations sent by Bob.


Size
Name32bit
length of set 8bit?

Atleast 40bit

