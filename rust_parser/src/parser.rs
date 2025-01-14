peg::parser!{
  pub grammar time_parser() for str {
    rule digit() -> u32
      = n:$(['0'..='9']) {? n.parse().or(Err("u32")) }

    rule hour() -> u32
    = dig:$(digit() digit()) {? dig.parse().or(Err("u32")) } /
      dig:$(digit()) {? dig.parse().or(Err("u32")) }

    rule minute() -> u32
          = dig:$(digit() digit()) {? dig.parse().or(Err("u32")) }

    rule ampm() -> u32
      = "am" { 0 } /
        "pm" { 12 }
    
    pub rule list() -> Vec<u32>
      = "[" l:(digit() ** ",") "]" { l }

    pub rule time() -> u32
      = hr:hour() " " offset:ampm() {
        hr*60 + offset*60
      } /
      hr:hour() ":" min:minute() " " offset:ampm() {
        hr*60 + offset*60 + min
      } /
      hr:hour() ":" min:minute(){
        hr*60 + min
      } 
  }
}