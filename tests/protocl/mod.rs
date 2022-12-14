use rdht::protocl::KRPC;
use rdht::protocl::{DHTQuery, DHTResponse};

#[test]
fn test_ping_decode() {
    let ping = KRPC::decode("d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe");
    assert_eq!(
        ping,
        Ok(KRPC::Query(
            "aa".into(),
            DHTQuery::Ping {
                id: "abcdefghij0123456789".to_string()
            }
        ))
    );
    let ping = KRPC::decode("d1:rd2:id20:mnopqrstuvwxyz123456e1:t2:aa1:y1:re");
    assert_eq!(
        ping,
        Ok(KRPC::Response(
            "aa".into(),
            DHTResponse::ID {
                id: "mnopqrstuvwxyz123456".to_string()
            }
        ))
    );
}

#[test]
fn test_find_node_decode() {
    let find_node = KRPC::decode("d1:ad2:id20:abcdefghij01234567896:target20:mnopqrstuvwxyz123456e1:q9:find_node1:t2:aa1:y1:qe");
    assert_eq!(
        find_node,
        Ok(KRPC::Query(
            "aa".into(),
            DHTQuery::FindNode {
                id: "abcdefghij0123456789".to_string(),
                target: "mnopqrstuvwxyz123456".to_string(),
            }
        ))
    );
    let find_node =
        KRPC::decode("d1:rd2:id20:0123456789abcdefghij5:nodes9:def456...e1:t2:aa1:y1:re");
    assert_eq!(
        find_node,
        Ok(KRPC::Response(
            "aa".into(),
            DHTResponse::FindNode {
                id: "0123456789abcdefghij".to_string(),
                nodes: "def456...".to_string(),
            }
        ))
    );
}

#[test]
fn test_announce_peer_decode() {
    let announce_peer = KRPC::decode("d1:ad2:id20:abcdefghij012345678912:implied_porti1e9:info_hash20:mnopqrstuvwxyz1234564:porti6881e5:token8:aoeusnthe1:q13:announce_peer1:t2:aa1:y1:qe");
    assert_eq!(
        announce_peer,
        Ok(KRPC::Query(
            "aa".into(),
            DHTQuery::AnnouncePeer {
                id: "abcdefghij0123456789".to_string(),
                impiled_port: 1,
                port: 6881,
                info_hash: "mnopqrstuvwxyz123456".to_string(),
                token: "aoeusnth".to_string()
            }
        ))
    );
    let announce_peer = KRPC::decode("d1:rd2:id20:mnopqrstuvwxyz123456e1:t2:aa1:y1:re");
    assert_eq!(
        announce_peer,
        Ok(KRPC::Response(
            "aa".into(),
            DHTResponse::ID {
                id: "mnopqrstuvwxyz123456".to_string()
            }
        ))
    );
}

#[test]
fn test_error_decode() {
    let error = KRPC::decode("d1:eli201e23:A Generic Error Ocurrede1:t2:aa1:y1:ee");
    assert_eq!(
        error,
        Ok(KRPC::Error(201, "A Generic Error Ocurred".to_string()))
    );
}
