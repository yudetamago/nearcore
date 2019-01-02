/// Creates HashMap.
/// # Examples:
///
/// ```
/// let m = map!{0 => 1, 2 => 3};
/// assert_eq!(m.len(), 2);
/// ```
#[allow(unused_macros)]
macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

/// Creates HashSet.
/// # Examples:
///
/// ```
/// let s = set!{2, 1};
/// assert_eq!(s.len(), 2);
/// ```
#[allow(unused_macros)]
macro_rules! set(
        { $($el:expr),+ } => {
            {
                let mut s = ::std::collections::HashSet::new();
                $(
                    s.insert($el);
                )+
                s
            }
        };
    );

/// Binds a tuple to a vector.
/// # Examples:
///
/// ```
/// let v = vec![1,2,3];
/// tuplet!((a,b,c) = v);
/// assert_eq!(a, &1);
/// assert_eq!(b, &2);
/// assert_eq!(c, &3);
/// ```
#[allow(unused_macros)]
macro_rules! tuplet {
    { ($y:ident $(, $x:ident)*) = $v:expr } => {
        let ($y, $($x),*) = tuplet!($v ; 1 ; ($($x),*) ; (&$v[0]) );
    };
    { $v:expr ; $j:expr ; ($y:ident $(, $x:ident)*) ; ($($a:expr),*) } => {
        tuplet!( $v ; $j+1 ; ($($x),*) ; ($($a),*,&$v[$j]) )
    };
    { $v:expr ; $j:expr ; () ; $accu:expr } => {
        $accu
    }
}

#[cfg(test)]
mod tests {
    use primitives::traits::{Payload, WitnessSelector};
    use primitives::types::UID;
    use std::collections::HashSet;
    use typed_arena::Arena;
    use dag::message::Message;

    struct FakeNonContinuousWitnessSelector {
        num_users: u64,
        users: HashSet<UID>,
    }

    impl FakeNonContinuousWitnessSelector {
        fn new(num_users: u64) -> Self {
            let mut users = set!{0};
            for i in 1..num_users {
                users.insert(i);
            }
            Self { num_users, users }
        }
    }

    impl WitnessSelector for FakeNonContinuousWitnessSelector {
        fn epoch_witnesses(&self, _epoch: u64) -> &HashSet<UID> {
            &self.users
        }
        fn epoch_leader(&self, epoch: u64) -> UID {
            epoch % self.num_users
        }
        fn random_witnesses(&self, _epoch: u64, _sample_size: usize) -> HashSet<UID> {
            unimplemented!()
        }
    }

    fn make_assertions<P: Payload>(messages: &[Option<&Message<P>>], assertions: &[(u64, Option<u64>, bool, u64)]) {
        for it in messages.iter().zip(assertions.iter()) {
            let (msg, a) = it;
            if let Some(msg) = msg.as_ref() {
                // If this assert triggers, the last element of tuples indicates the node uid
                assert_eq!((a.0, a.1, a.2, a.3), (msg.computed_epoch, msg.computed_is_representative, msg.computed_is_kickout, a.3));
            }
        }
    }

    fn epoch_representative_approved_by<P: Payload>(message: &Message<P>, epoch: u64, owner: u64) -> bool {
       message.approved_endorsements.contains_any_approval(epoch, owner) ||
           (owner == message.data.body.owner_uid && message.computed_endorsements.contains_epoch(epoch)) ||
           (owner == message.data.body.owner_uid && Some(epoch) == message.computed_is_representative)
    }

    fn test_endorsements<P: Payload>(message: &Message<P>, endorsements: &[Vec<u64>], num_users: u64) {
        for epoch in 0..endorsements.len() {
            let who = &endorsements[epoch];
            let mut lst = 0;

            for i in 0..num_users {
                if lst < who.len() && who[lst] == i {
                    assert!(epoch_representative_approved_by(message, epoch as u64, i), "epoch: {}, i: {}, {:?}", epoch, i, message.computed_endorsements);
                    lst += 1;
                }
                else {
                    assert!(!epoch_representative_approved_by(message, epoch as u64, i), "epoch: {}, i: {}", epoch, i);
                }
            }
        }
    }
    #[test]
    fn generated_1548290541() {
         /* Test generated @1548290541 */

         /* false */
         let arena = Arena::new();
         let selector = FakeNonContinuousWitnessSelector::new(6);
         let (v1,v0,v4,v3,v2,v5,v11,v6,v8,v12,v7,v23,v9,v13,v10,v14,v18,v20,v15,v22,v26,v45,v16,v19,v17,v21,v25,v31,v24,v28,v34,v27,v29,v37,v35,v30,v36,v32,v39,v33,v38,v42,v40,v41,v47,v43,v46,v49,v44,v52,v48,v53,v50,v55,v51,v58,v60,v54,v61,v56,v66,v63,v57,v68,v59,v69,v62,v64,v65,v67,v71,v74,v70,v72,v73);
         let mut v = [None; 75];
         let test = |v:&[_]| make_assertions(&v, &[(0, Some(undefined), undefined, 1), (0, Some(undefined), undefined, 0), (0, Some(undefined), undefined, 4), (0, Some(0), undefined, 3), (0, Some(undefined), undefined, 2), (0, Some(undefined), undefined, 5), (0, Some(undefined), undefined, 11), (0, Some(undefined), undefined, 6), (0, Some(undefined), undefined, 8), (0, Some(undefined), undefined, 12), (0, Some(undefined), undefined, 7), (0, Some(0), undefined, 23), (0, Some(undefined), undefined, 9), (0, Some(undefined), undefined, 13), (0, Some(undefined), undefined, 10), (0, Some(undefined), undefined, 14), (1, Some(undefined), undefined, 18), (1, Some(1), undefined, 20), (1, Some(undefined), undefined, 15), (1, Some(undefined), undefined, 22), (1, Some(undefined), undefined, 26), (1, Some(undefined), undefined, 45), (1, Some(undefined), undefined, 16), (1, Some(undefined), undefined, 19), (1, Some(undefined), undefined, 17), (1, Some(undefined), undefined, 21), (1, Some(undefined), undefined, 25), (1, Some(undefined), undefined, 31), (1, Some(undefined), undefined, 24), (1, Some(undefined), undefined, 28), (2, Some(undefined), undefined, 34), (2, Some(undefined), undefined, 27), (2, Some(undefined), undefined, 29), (1, Some(undefined), undefined, 37), (2, Some(undefined), undefined, 35), (2, Some(undefined), undefined, 30), (2, Some(undefined), undefined, 36), (2, Some(undefined), undefined, 32), (2, Some(undefined), undefined, 39), (2, Some(undefined), undefined, 33), (2, Some(undefined), undefined, 38), (2, Some(undefined), undefined, 42), (2, Some(undefined), undefined, 40), (2, Some(undefined), undefined, 41), (2, Some(undefined), undefined, 47), (2, Some(undefined), undefined, 43), (2, Some(undefined), undefined, 46), (2, Some(undefined), undefined, 49), (2, Some(undefined), undefined, 44), (3, Some(undefined), undefined, 52), (2, Some(undefined), undefined, 48), (3, Some(undefined), undefined, 53), (2, Some(undefined), undefined, 50), (3, Some(undefined), undefined, 55), (3, Some(undefined), undefined, 51), (3, Some(undefined), undefined, 58), (3, Some(undefined), undefined, 60), (3, Some(undefined), undefined, 54), (3, Some(undefined), undefined, 61), (3, Some(undefined), undefined, 56), (3, Some(undefined), undefined, 66), (3, Some(undefined), undefined, 63), (3, Some(undefined), undefined, 57), (3, Some(undefined), undefined, 68), (3, Some(undefined), undefined, 59), (3, Some(undefined), undefined, 69), (3, Some(undefined), undefined, 62), (3, Some(undefined), undefined, 64), (3, Some(undefined), undefined, 65), (3, Some(undefined), undefined, 67), (3, Some(undefined), undefined, 71), (3, Some(undefined), undefined, 74), (3, Some(undefined), undefined, 70), (3, Some(undefined), undefined, 72), (3, Some(undefined), undefined, 73)]);
         simple_messages!(0, &selector, arena [5, 0, true => v1;]); v[0] = Some(v1);
         test(&v);
         test_endorsements(v1, &[], 6);
         simple_messages!(0, &selector, arena [2, 0, true => v0;]); v[1] = Some(v0);
         test(&v);
         test_endorsements(v0, &[], 6);
         simple_messages!(0, &selector, arena [1, 0, true => v4;]); v[2] = Some(v4);
         test(&v);
         test_endorsements(v4, &[], 6);
         simple_messages!(0, &selector, arena [0, 0, true => v3;]); v[3] = Some(v3);
         test(&v);
         test_endorsements(v3, &[], 6);
         simple_messages!(0, &selector, arena [[=> v1; ] => 5, 0, true => v2;]); v[4] = Some(v2);
         test(&v);
         test_endorsements(v2, &[], 6);
         simple_messages!(0, &selector, arena [[=> v0; ] => 3, 0, true => v5;]); v[5] = Some(v5);
         test(&v);
         test_endorsements(v5, &[], 6);
         simple_messages!(0, &selector, arena [[=> v0; => v1; ] => 2, 0, true => v11;]); v[6] = Some(v11);
         test(&v);
         test_endorsements(v11, &[], 6);
         simple_messages!(0, &selector, arena [[=> v4; ] => 1, 0, true => v6;]); v[7] = Some(v6);
         test(&v);
         test_endorsements(v6, &[], 6);
         simple_messages!(0, &selector, arena [[=> v5; ] => 3, 0, true => v8;]); v[8] = Some(v8);
         test(&v);
         test_endorsements(v8, &[], 6);
         simple_messages!(0, &selector, arena [[=> v11; ] => 2, 0, true => v12;]); v[9] = Some(v12);
         test(&v);
         test_endorsements(v12, &[], 6);
         simple_messages!(0, &selector, arena [[=> v0; => v6; ] => 1, 0, true => v7;]); v[10] = Some(v7);
         test(&v);
         test_endorsements(v7, &[], 6);
         simple_messages!(0, &selector, arena [[=> v6; ] => 0, 0, true => v23;]); v[11] = Some(v23);
         test(&v);
         test_endorsements(v23, &[], 6);
         simple_messages!(0, &selector, arena [[=> v4; => v8; ] => 3, 0, true => v9;]); v[12] = Some(v9);
         test(&v);
         test_endorsements(v9, &[], 6);
         simple_messages!(0, &selector, arena [[=> v12; ] => 2, 0, true => v13;]); v[13] = Some(v13);
         test(&v);
         test_endorsements(v13, &[], 6);
         simple_messages!(0, &selector, arena [[=> v2; => v3; => v7; => v8; ] => 4, 0, true => v10;]); v[14] = Some(v10);
         test(&v);
         test_endorsements(v10, &[], 6);
         simple_messages!(0, &selector, arena [[=> v3; => v13; ] => 2, 0, true => v14;]); v[15] = Some(v14);
         test(&v);
         test_endorsements(v14, &[], 6);
         simple_messages!(0, &selector, arena [[=> v4; => v5; => v10; ] => 0, 0, true => v18;]); v[16] = Some(v18);
         test(&v);
         test_endorsements(v18, &[], 6);
         simple_messages!(0, &selector, arena [[=> v5; => v10; ] => 1, 0, true => v20;]); v[17] = Some(v20);
         test(&v);
         test_endorsements(v20, &[], 6);
         simple_messages!(0, &selector, arena [[=> v9; => v14; ] => 2, 0, true => v15;]); v[18] = Some(v15);
         test(&v);
         test_endorsements(v15, &[], 6);
         simple_messages!(0, &selector, arena [[=> v6; => v13; => v18; ] => 0, 0, true => v22;]); v[19] = Some(v22);
         test(&v);
         test_endorsements(v22, &[], 6);
         simple_messages!(0, &selector, arena [[=> v9; => v11; => v20; ] => 1, 0, true => v26;]); v[20] = Some(v26);
         test(&v);
         test_endorsements(v26, &[], 6);
         simple_messages!(0, &selector, arena [[=> v20; ] => 0, 0, true => v45;]); v[21] = Some(v45);
         test(&v);
         test_endorsements(v45, &[], 6);
         simple_messages!(0, &selector, arena [[=> v2; => v15; ] => 2, 0, true => v16;]); v[22] = Some(v16);
         test(&v);
         test_endorsements(v16, &[], 6);
         simple_messages!(0, &selector, arena [[=> v2; => v5; => v6; => v11; => v15; ] => 5, 0, true => v19;]); v[23] = Some(v19);
         test(&v);
         test_endorsements(v19, &[], 6);
         simple_messages!(0, &selector, arena [[=> v16; ] => 2, 0, true => v17;]); v[24] = Some(v17);
         test(&v);
         test_endorsements(v17, &[], 6);
         simple_messages!(0, &selector, arena [[=> v3; => v7; => v19; ] => 5, 0, true => v21;]); v[25] = Some(v21);
         test(&v);
         test_endorsements(v21, &[], 6);
         simple_messages!(0, &selector, arena [[=> v6; => v11; => v15; => v18; => v19; => v23; ] => 0, 0, true => v25;]); v[26] = Some(v25);
         test(&v);
         test_endorsements(v25, &[], 6);
         simple_messages!(0, &selector, arena [[=> v17; => v18; => v19; ] => 3, 0, true => v31;]); v[27] = Some(v31);
         test(&v);
         test_endorsements(v31, &[], 6);
         simple_messages!(0, &selector, arena [[=> v17; => v18; => v20; => v21; => v23; ] => 4, 0, true => v24;]); v[28] = Some(v24);
         test(&v);
         test_endorsements(v24, &[], 6);
         simple_messages!(0, &selector, arena [[=> v12; => v16; => v25; => v26; ] => 1, 0, true => v28;]); v[29] = Some(v28);
         test(&v);
         test_endorsements(v28, &[], 6);
         simple_messages!(0, &selector, arena [[=> v20; => v22; => v25; => v31; ] => 3, 0, true => v34;]); v[30] = Some(v34);
         test(&v);
         test_endorsements(v34, &[], 6);
         simple_messages!(0, &selector, arena [[=> v0; => v1; => v2; => v3; => v4; => v5; => v6; => v7; => v8; => v9; => v10; => v11; => v12; => v13; => v14; => v15; => v16; => v17; => v18; => v19; => v20; => v21; => v22; => v23; => v24; => v25; => v26; ] => 0, 0, true => v27;]); v[31] = Some(v27);
         test(&v);
         test_endorsements(v27, &[], 6);
         simple_messages!(0, &selector, arena [[=> v22; => v24; => v25; => v26; ] => 4, 0, true => v29;]); v[32] = Some(v29);
         test(&v);
         test_endorsements(v29, &[], 6);
         simple_messages!(0, &selector, arena [[=> v17; => v20; => v21; => v22; => v23; => v28; ] => 5, 0, true => v37;]); v[33] = Some(v37);
         test(&v);
         test_endorsements(v37, &[], 6);
         simple_messages!(0, &selector, arena [[=> v34; ] => 3, 0, true => v35;]); v[34] = Some(v35);
         test(&v);
         test_endorsements(v35, &[], 6);
         simple_messages!(0, &selector, arena [[=> v29; ] => 4, 0, true => v30;]); v[35] = Some(v30);
         test(&v);
         test_endorsements(v30, &[], 6);
         simple_messages!(0, &selector, arena [[=> v21; => v35; ] => 3, 0, true => v36;]); v[36] = Some(v36);
         test(&v);
         test_endorsements(v36, &[], 6);
         simple_messages!(0, &selector, arena [[=> v27; => v28; => v30; ] => 4, 0, true => v32;]); v[37] = Some(v32);
         test(&v);
         test_endorsements(v32, &[], 6);
         simple_messages!(0, &selector, arena [[=> v27; => v28; => v30; ] => 1, 0, true => v39;]); v[38] = Some(v39);
         test(&v);
         test_endorsements(v39, &[], 6);
         simple_messages!(0, &selector, arena [[=> v32; ] => 4, 0, true => v33;]); v[39] = Some(v33);
         test(&v);
         test_endorsements(v33, &[], 6);
         simple_messages!(0, &selector, arena [[=> v24; => v32; => v36; ] => 3, 0, true => v38;]); v[40] = Some(v38);
         test(&v);
         test_endorsements(v38, &[], 6);
         simple_messages!(0, &selector, arena [[=> v1; => v7; => v9; => v10; => v21; => v24; => v27; => v29; => v30; => v31; => v32; => v33; => v34; => v35; ] => 0, 0, true => v42;]); v[41] = Some(v42);
         test(&v);
         test_endorsements(v42, &[], 6);
         simple_messages!(0, &selector, arena [[=> v38; ] => 3, 0, true => v40;]); v[42] = Some(v40);
         test(&v);
         test_endorsements(v40, &[], 6);
         simple_messages!(0, &selector, arena [[=> v29; => v40; ] => 3, 0, true => v41;]); v[43] = Some(v41);
         test(&v);
         test_endorsements(v41, &[], 6);
         simple_messages!(0, &selector, arena [[=> v40; ] => 0, 0, true => v47;]); v[44] = Some(v47);
         test(&v);
         test_endorsements(v47, &[], 6);
         simple_messages!(0, &selector, arena [[=> v41; ] => 3, 0, true => v43;]); v[45] = Some(v43);
         test(&v);
         test_endorsements(v43, &[], 6);
         simple_messages!(0, &selector, arena [[=> v33; => v36; => v37; => v39; => v41; ] => 5, 0, true => v46;]); v[46] = Some(v46);
         test(&v);
         test_endorsements(v46, &[], 6);
         simple_messages!(0, &selector, arena [[=> v36; => v39; => v42; => v45; => v47; ] => 0, 0, true => v49;]); v[47] = Some(v49);
         test(&v);
         test_endorsements(v49, &[], 6);
         simple_messages!(0, &selector, arena [[=> v33; => v43; ] => 3, 0, true => v44;]); v[48] = Some(v44);
         test(&v);
         test_endorsements(v44, &[], 6);
         simple_messages!(0, &selector, arena [[=> v38; => v42; => v46; ] => 5, 0, true => v52;]); v[49] = Some(v52);
         test(&v);
         test_endorsements(v52, &[], 6);
         simple_messages!(0, &selector, arena [[=> v37; => v44; ] => 3, 0, true => v48;]); v[50] = Some(v48);
         test(&v);
         test_endorsements(v48, &[], 6);
         simple_messages!(0, &selector, arena [[=> v52; ] => 5, 0, true => v53;]); v[51] = Some(v53);
         test(&v);
         test_endorsements(v53, &[], 6);
         simple_messages!(0, &selector, arena [[=> v48; ] => 3, 0, true => v50;]); v[52] = Some(v50);
         test(&v);
         test_endorsements(v50, &[], 6);
         simple_messages!(0, &selector, arena [[=> v43; => v49; => v53; ] => 5, 0, true => v55;]); v[53] = Some(v55);
         test(&v);
         test_endorsements(v55, &[], 6);
         simple_messages!(0, &selector, arena [[=> v45; => v46; => v50; ] => 3, 0, true => v51;]); v[54] = Some(v51);
         test(&v);
         test_endorsements(v51, &[], 6);
         simple_messages!(0, &selector, arena [[=> v44; => v50; => v55; ] => 5, 0, true => v58;]); v[55] = Some(v58);
         test(&v);
         test_endorsements(v58, &[], 6);
         simple_messages!(0, &selector, arena [[=> v44; => v46; => v55; ] => 0, 0, true => v60;]); v[56] = Some(v60);
         test(&v);
         test_endorsements(v60, &[], 6);
         simple_messages!(0, &selector, arena [[=> v51; ] => 3, 0, true => v54;]); v[57] = Some(v54);
         test(&v);
         test_endorsements(v54, &[], 6);
         simple_messages!(0, &selector, arena [[=> v51; => v58; ] => 5, 0, true => v61;]); v[58] = Some(v61);
         test(&v);
         test_endorsements(v61, &[], 6);
         simple_messages!(0, &selector, arena [[=> v54; ] => 3, 0, true => v56;]); v[59] = Some(v56);
         test(&v);
         test_endorsements(v56, &[], 6);
         simple_messages!(0, &selector, arena [[=> v54; => v60; ] => 0, 0, true => v66;]); v[60] = Some(v66);
         test(&v);
         test_endorsements(v66, &[], 6);
         simple_messages!(0, &selector, arena [[=> v61; ] => 5, 0, true => v63;]); v[61] = Some(v63);
         test(&v);
         test_endorsements(v63, &[], 6);
         simple_messages!(0, &selector, arena [[=> v56; ] => 3, 0, true => v57;]); v[62] = Some(v57);
         test(&v);
         test_endorsements(v57, &[], 6);
         simple_messages!(0, &selector, arena [[=> v56; => v60; => v63; ] => 5, 0, true => v68;]); v[63] = Some(v68);
         test(&v);
         test_endorsements(v68, &[], 6);
         simple_messages!(0, &selector, arena [[=> v42; => v57; ] => 3, 0, true => v59;]); v[64] = Some(v59);
         test(&v);
         test_endorsements(v59, &[], 6);
         simple_messages!(0, &selector, arena [[=> v68; ] => 5, 0, true => v69;]); v[65] = Some(v69);
         test(&v);
         test_endorsements(v69, &[], 6);
         simple_messages!(0, &selector, arena [[=> v53; => v59; ] => 3, 0, true => v62;]); v[66] = Some(v62);
         test(&v);
         test_endorsements(v62, &[], 6);
         simple_messages!(0, &selector, arena [[=> v47; => v62; ] => 3, 0, true => v64;]); v[67] = Some(v64);
         test(&v);
         test_endorsements(v64, &[], 6);
         simple_messages!(0, &selector, arena [[=> v59; => v60; => v63; => v64; ] => 4, 0, true => v65;]); v[68] = Some(v65);
         test(&v);
         test_endorsements(v65, &[], 6);
         simple_messages!(0, &selector, arena [[=> v55; => v64; ] => 3, 0, true => v67;]); v[69] = Some(v67);
         test(&v);
         test_endorsements(v67, &[], 6);
         simple_messages!(0, &selector, arena [[=> v64; => v66; ] => 0, 0, true => v71;]); v[70] = Some(v71);
         test(&v);
         test_endorsements(v71, &[], 6);
         simple_messages!(0, &selector, arena [[=> v57; => v62; => v65; => v69; ] => 5, 0, true => v74;]); v[71] = Some(v74);
         test(&v);
         test_endorsements(v74, &[], 6);
         simple_messages!(0, &selector, arena [[=> v65; => v66; => v67; => v69; ] => 4, 0, true => v70;]); v[72] = Some(v70);
         test(&v);
         test_endorsements(v70, &[], 6);
         simple_messages!(0, &selector, arena [[=> v70; => v71; ] => 4, 0, true => v72;]); v[73] = Some(v72);
         test(&v);
         test_endorsements(v72, &[], 6);
         simple_messages!(0, &selector, arena [[=> v72; ] => 4, 0, true => v73;]); v[74] = Some(v73);
         test(&v);
         test_endorsements(v73, &[], 6);
    }
}