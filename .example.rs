   if content == "?hello" {
                        rev_send(data.token.clone(), channel.clone(), "world!".to_string()));

                    }else if content == "?ping" {
                        rev_send(data.token.clone(), channel.clone(), "Pong!!".to_string());
                    };        
