pub fn actions(n: u8) -> Vec<&'static str> {
    let mut handshake = Vec::new();
    let mut rest = n;
    if (rest & 1) == 1 { 
        handshake.push("wink"); 
    }
    rest >>= 1;
    if (rest & 1) == 1 { 
        handshake.push("double blink"); 
    }
    rest >>= 1;
    if (rest & 1) == 1 { 
        handshake.push("close your eyes"); 
    }
    rest >>= 1; 
    if (rest & 1) == 1 { 
        handshake.push("jump"); 
    }
    rest >>= 1;
    if (rest & 1) == 1 { 
        handshake.reverse() 
    }
    handshake
}