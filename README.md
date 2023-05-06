# Bend 

This repo is for reconciling Sets of Hashes that represent files. It is not a Data transfer protocol. It is a POC to test and build.

### Authenticated Ops using UCANs
Using UCANs


# Vector Clock
- BAO encoding of the Vector Clock
-


````
Structure {
    set_store: Btree or kd-tree TBD/will be sorted alphabetacally,
    items: (value, signature)
    event_clock: {
        Vclock {
            adds: int32
            deletes: int32,
        }
    }
}

Auth {
    signature: Hash(event_entry)
}
````
# Elements
Air Bend - Rust/ Cloud 
Fire Bend - Frontend js/ts/wasm
Earth bend - Stationary Iot embeded
Water Bend - Dynamic Nodes/ Mobile Embeded
