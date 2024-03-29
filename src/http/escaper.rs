fn html_entity(key: &str) -> String {
  match key {
    "&quot;" => String::from_utf8_lossy(b"\""),
    "&amp;" => String::from_utf8_lossy(b"&"),
    "&lt;" => String::from_utf8_lossy(b"<"),
    "&gt;" => String::from_utf8_lossy(b">"),
    "&apos;" => String::from_utf8_lossy(b"'"),
    "&nbsp;" => String::from_utf8_lossy(b"\xc2\xa0"),
    "&iexcl;" => String::from_utf8_lossy(b"\xc2\xa1"),
    "&cent;" => String::from_utf8_lossy(b"\xc2\xa2"),
    "&pound;" => String::from_utf8_lossy(b"\xc2\xa3"),
    "&curren;" => String::from_utf8_lossy(b"\xc2\xa4"),
    "&yen;" => String::from_utf8_lossy(b"\xc2\xa5"),
    "&brvbar;" => String::from_utf8_lossy(b"\xc2\xa6"),
    "&sect;" => String::from_utf8_lossy(b"\xc2\xa7"),
    "&uml;" => String::from_utf8_lossy(b"\xc2\xa8"),
    "&copy;" => String::from_utf8_lossy(b"\xc2\xa9"),
    "&ordf;" => String::from_utf8_lossy(b"\xc2\xaa"),
    "&laquo;" => String::from_utf8_lossy(b"\xc2\xab"),
    "&not;" => String::from_utf8_lossy(b"\xc2\xac"),
    "&shy;" => String::from_utf8_lossy(b"\xc2\xad"),
    "&reg;" => String::from_utf8_lossy(b"\xc2\xae"),
    "&macr;" => String::from_utf8_lossy(b"\xc2\xaf"),
    "&deg;" => String::from_utf8_lossy(b"\xc2\xb0"),
    "&plusmn;" => String::from_utf8_lossy(b"\xc2\xb1"),
    "&sup2;" => String::from_utf8_lossy(b"\xc2\xb2"),
    "&sup3;" => String::from_utf8_lossy(b"\xc2\xb3"),
    "&acute;" => String::from_utf8_lossy(b"\xc2\xb4"),
    "&micro;" => String::from_utf8_lossy(b"\xc2\xb5"),
    "&para;" => String::from_utf8_lossy(b"\xc2\xb6"),
    "&middot;" => String::from_utf8_lossy(b"\xc2\xb7"),
    "&cedil;" => String::from_utf8_lossy(b"\xc2\xb8"),
    "&sup1;" => String::from_utf8_lossy(b"\xc2\xb9"),
    "&ordm;" => String::from_utf8_lossy(b"\xc2\xba"),
    "&raquo;" => String::from_utf8_lossy(b"\xc2\xbb"),
    "&frac14;" => String::from_utf8_lossy(b"\xc2\xbc"),
    "&frac12;" => String::from_utf8_lossy(b"\xc2\xbd"),
    "&frac34;" => String::from_utf8_lossy(b"\xc2\xbe"),
    "&iquest;" => String::from_utf8_lossy(b"\xc2\xbf"),
    "&Agrave;" => String::from_utf8_lossy(b"\xc3\x80"),
    "&Aacute;" => String::from_utf8_lossy(b"\xc3\x81"),
    "&Acirc;" => String::from_utf8_lossy(b"\xc3\x82"),
    "&Atilde;" => String::from_utf8_lossy(b"\xc3\x83"),
    "&Auml;" => String::from_utf8_lossy(b"\xc3\x84"),
    "&Aring;" => String::from_utf8_lossy(b"\xc3\x85"),
    "&AElig;" => String::from_utf8_lossy(b"\xc3\x86"),
    "&Ccedil;" => String::from_utf8_lossy(b"\xc3\x87"),
    "&Egrave;" => String::from_utf8_lossy(b"\xc3\x88"),
    "&Eacute;" => String::from_utf8_lossy(b"\xc3\x89"),
    "&Ecirc;" => String::from_utf8_lossy(b"\xc3\x8a"),
    "&Euml;" => String::from_utf8_lossy(b"\xc3\x8b"),
    "&Igrave;" => String::from_utf8_lossy(b"\xc3\x8c"),
    "&Iacute;" => String::from_utf8_lossy(b"\xc3\x8d"),
    "&Icirc;" => String::from_utf8_lossy(b"\xc3\x8e"),
    "&Iuml;" => String::from_utf8_lossy(b"\xc3\x8f"),
    "&ETH;" => String::from_utf8_lossy(b"\xc3\x90"),
    "&Ntilde;" => String::from_utf8_lossy(b"\xc3\x91"),
    "&Ograve;" => String::from_utf8_lossy(b"\xc3\x92"),
    "&Oacute;" => String::from_utf8_lossy(b"\xc3\x93"),
    "&Ocirc;" => String::from_utf8_lossy(b"\xc3\x94"),
    "&Otilde;" => String::from_utf8_lossy(b"\xc3\x95"),
    "&Ouml;" => String::from_utf8_lossy(b"\xc3\x96"),
    "&times;" => String::from_utf8_lossy(b"\xc3\x97"),
    "&Oslash;" => String::from_utf8_lossy(b"\xc3\x98"),
    "&Ugrave;" => String::from_utf8_lossy(b"\xc3\x99"),
    "&Uacute;" => String::from_utf8_lossy(b"\xc3\x9a"),
    "&Ucirc;" => String::from_utf8_lossy(b"\xc3\x9b"),
    "&Uuml;" => String::from_utf8_lossy(b"\xc3\x9c"),
    "&Yacute;" => String::from_utf8_lossy(b"\xc3\x9d"),
    "&THORN;" => String::from_utf8_lossy(b"\xc3\x9e"),
    "&szlig;" => String::from_utf8_lossy(b"\xc3\x9f"),
    "&agrave;" => String::from_utf8_lossy(b"\xc3\xa0"),
    "&aacute;" => String::from_utf8_lossy(b"\xc3\xa1"),
    "&acirc;" => String::from_utf8_lossy(b"\xc3\xa2"),
    "&atilde;" => String::from_utf8_lossy(b"\xc3\xa3"),
    "&auml;" => String::from_utf8_lossy(b"\xc3\xa4"),
    "&aring;" => String::from_utf8_lossy(b"\xc3\xa5"),
    "&aelig;" => String::from_utf8_lossy(b"\xc3\xa6"),
    "&ccedil;" => String::from_utf8_lossy(b"\xc3\xa7"),
    "&egrave;" => String::from_utf8_lossy(b"\xc3\xa8"),
    "&eacute;" => String::from_utf8_lossy(b"\xc3\xa9"),
    "&ecirc;" => String::from_utf8_lossy(b"\xc3\xaa"),
    "&euml;" => String::from_utf8_lossy(b"\xc3\xab"),
    "&igrave;" => String::from_utf8_lossy(b"\xc3\xac"),
    "&iacute;" => String::from_utf8_lossy(b"\xc3\xad"),
    "&icirc;" => String::from_utf8_lossy(b"\xc3\xae"),
    "&iuml;" => String::from_utf8_lossy(b"\xc3\xaf"),
    "&eth;" => String::from_utf8_lossy(b"\xc3\xb0"),
    "&ntilde;" => String::from_utf8_lossy(b"\xc3\xb1"),
    "&ograve;" => String::from_utf8_lossy(b"\xc3\xb2"),
    "&oacute;" => String::from_utf8_lossy(b"\xc3\xb3"),
    "&ocirc;" => String::from_utf8_lossy(b"\xc3\xb4"),
    "&otilde;" => String::from_utf8_lossy(b"\xc3\xb5"),
    "&ouml;" => String::from_utf8_lossy(b"\xc3\xb6"),
    "&divide;" => String::from_utf8_lossy(b"\xc3\xb7"),
    "&oslash;" => String::from_utf8_lossy(b"\xc3\xb8"),
    "&ugrave;" => String::from_utf8_lossy(b"\xc3\xb9"),
    "&uacute;" => String::from_utf8_lossy(b"\xc3\xba"),
    "&ucirc;" => String::from_utf8_lossy(b"\xc3\xbb"),
    "&uuml;" => String::from_utf8_lossy(b"\xc3\xbc"),
    "&yacute;" => String::from_utf8_lossy(b"\xc3\xbd"),
    "&thorn;" => String::from_utf8_lossy(b"\xc3\xbe"),
    "&yuml;" => String::from_utf8_lossy(b"\xc3\xbf"),
    "&OElig;" => String::from_utf8_lossy(b"\xc5\x92"),
    "&oelig;" => String::from_utf8_lossy(b"\xc5\x93"),
    "&Scaron;" => String::from_utf8_lossy(b"\xc5\xa0"),
    "&scaron;" => String::from_utf8_lossy(b"\xc5\xa1"),
    "&Yuml;" => String::from_utf8_lossy(b"\xc5\xb8"),
    "&fnof;" => String::from_utf8_lossy(b"\xc6\x92"),
    "&circ;" => String::from_utf8_lossy(b"\xcb\x86"),
    "&tilde;" => String::from_utf8_lossy(b"\xcb\x9c"),
    "&Alpha;" => String::from_utf8_lossy(b"\xce\x91"),
    "&Beta;" => String::from_utf8_lossy(b"\xce\x92"),
    "&Gamma;" => String::from_utf8_lossy(b"\xce\x93"),
    "&Delta;" => String::from_utf8_lossy(b"\xce\x94"),
    "&Epsilon;" => String::from_utf8_lossy(b"\xce\x95"),
    "&Zeta;" => String::from_utf8_lossy(b"\xce\x96"),
    "&Eta;" => String::from_utf8_lossy(b"\xce\x97"),
    "&Theta;" => String::from_utf8_lossy(b"\xce\x98"),
    "&Iota;" => String::from_utf8_lossy(b"\xce\x99"),
    "&Kappa;" => String::from_utf8_lossy(b"\xce\x9a"),
    "&Lambda;" => String::from_utf8_lossy(b"\xce\x9b"),
    "&Mu;" => String::from_utf8_lossy(b"\xce\x9c"),
    "&Nu;" => String::from_utf8_lossy(b"\xce\x9d"),
    "&Xi;" => String::from_utf8_lossy(b"\xce\x9e"),
    "&Omicron;" => String::from_utf8_lossy(b"\xce\x9f"),
    "&Pi;" => String::from_utf8_lossy(b"\xce\xa0"),
    "&Rho;" => String::from_utf8_lossy(b"\xce\xa1"),
    "&Sigma;" => String::from_utf8_lossy(b"\xce\xa3"),
    "&Tau;" => String::from_utf8_lossy(b"\xce\xa4"),
    "&Upsilon;" => String::from_utf8_lossy(b"\xce\xa5"),
    "&Phi;" => String::from_utf8_lossy(b"\xce\xa6"),
    "&Chi;" => String::from_utf8_lossy(b"\xce\xa7"),
    "&Psi;" => String::from_utf8_lossy(b"\xce\xa8"),
    "&Omega;" => String::from_utf8_lossy(b"\xce\xa9"),
    "&alpha;" => String::from_utf8_lossy(b"\xce\xb1"),
    "&beta;" => String::from_utf8_lossy(b"\xce\xb2"),
    "&gamma;" => String::from_utf8_lossy(b"\xce\xb3"),
    "&delta;" => String::from_utf8_lossy(b"\xce\xb4"),
    "&epsilon;" => String::from_utf8_lossy(b"\xce\xb5"),
    "&zeta;" => String::from_utf8_lossy(b"\xce\xb6"),
    "&eta;" => String::from_utf8_lossy(b"\xce\xb7"),
    "&theta;" => String::from_utf8_lossy(b"\xce\xb8"),
    "&iota;" => String::from_utf8_lossy(b"\xce\xb9"),
    "&kappa;" => String::from_utf8_lossy(b"\xce\xba"),
    "&lambda;" => String::from_utf8_lossy(b"\xce\xbb"),
    "&mu;" => String::from_utf8_lossy(b"\xce\xbc"),
    "&nu;" => String::from_utf8_lossy(b"\xce\xbd"),
    "&xi;" => String::from_utf8_lossy(b"\xce\xbe"),
    "&omicron;" => String::from_utf8_lossy(b"\xce\xbf"),
    "&pi;" => String::from_utf8_lossy(b"\xcf\x80"),
    "&rho;" => String::from_utf8_lossy(b"\xcf\x81"),
    "&sigmaf;" => String::from_utf8_lossy(b"\xcf\x82"),
    "&sigma;" => String::from_utf8_lossy(b"\xcf\x83"),
    "&tau;" => String::from_utf8_lossy(b"\xcf\x84"),
    "&upsilon;" => String::from_utf8_lossy(b"\xcf\x85"),
    "&phi;" => String::from_utf8_lossy(b"\xcf\x86"),
    "&chi;" => String::from_utf8_lossy(b"\xcf\x87"),
    "&psi;" => String::from_utf8_lossy(b"\xcf\x88"),
    "&omega;" => String::from_utf8_lossy(b"\xcf\x89"),
    "&thetasym;" => String::from_utf8_lossy(b"\xcf\x91"),
    "&upsih;" => String::from_utf8_lossy(b"\xcf\x92"),
    "&piv;" => String::from_utf8_lossy(b"\xcf\x96"),
    "&ensp;" => String::from_utf8_lossy(b"\xe2\x80\x82"),
    "&emsp;" => String::from_utf8_lossy(b"\xe2\x80\x83"),
    "&thinsp;" => String::from_utf8_lossy(b"\xe2\x80\x89"),
    "&zwnj;" => String::from_utf8_lossy(b"\xe2\x80\x8c"),
    "&zwj;" => String::from_utf8_lossy(b"\xe2\x80\x8d"),
    "&lrm;" => String::from_utf8_lossy(b"\xe2\x80\x8e"),
    "&rlm;" => String::from_utf8_lossy(b"\xe2\x80\x8f"),
    "&ndash;" => String::from_utf8_lossy(b"\xe2\x80\x93"),
    "&mdash;" => String::from_utf8_lossy(b"\xe2\x80\x94"),
    "&lsquo;" => String::from_utf8_lossy(b"\xe2\x80\x98"),
    "&rsquo;" => String::from_utf8_lossy(b"\xe2\x80\x99"),
    "&sbquo;" => String::from_utf8_lossy(b"\xe2\x80\x9a"),
    "&ldquo;" => String::from_utf8_lossy(b"\xe2\x80\x9c"),
    "&rdquo;" => String::from_utf8_lossy(b"\xe2\x80\x9d"),
    "&bdquo;" => String::from_utf8_lossy(b"\xe2\x80\x9e"),
    "&dagger;" => String::from_utf8_lossy(b"\xe2\x80\xa0"),
    "&Dagger;" => String::from_utf8_lossy(b"\xe2\x80\xa1"),
    "&bull;" => String::from_utf8_lossy(b"\xe2\x80\xa2"),
    "&hellip;" => String::from_utf8_lossy(b"\xe2\x80\xa6"),
    "&permil;" => String::from_utf8_lossy(b"\xe2\x80\xb0"),
    "&prime;" => String::from_utf8_lossy(b"\xe2\x80\xb2"),
    "&Prime;" => String::from_utf8_lossy(b"\xe2\x80\xb3"),
    "&lsaquo;" => String::from_utf8_lossy(b"\xe2\x80\xb9"),
    "&rsaquo;" => String::from_utf8_lossy(b"\xe2\x80\xba"),
    "&oline;" => String::from_utf8_lossy(b"\xe2\x80\xbe"),
    "&frasl;" => String::from_utf8_lossy(b"\xe2\x81\x84"),
    "&euro;" => String::from_utf8_lossy(b"\xe2\x82\xac"),
    "&image;" => String::from_utf8_lossy(b"\xe2\x84\x91"),
    "&weierp;" => String::from_utf8_lossy(b"\xe2\x84\x98"),
    "&real;" => String::from_utf8_lossy(b"\xe2\x84\x9c"),
    "&trade;" => String::from_utf8_lossy(b"\xe2\x84\xa2"),
    "&alefsym;" => String::from_utf8_lossy(b"\xe2\x84\xb5"),
    "&larr;" => String::from_utf8_lossy(b"\xe2\x86\x90"),
    "&uarr;" => String::from_utf8_lossy(b"\xe2\x86\x91"),
    "&rarr;" => String::from_utf8_lossy(b"\xe2\x86\x92"),
    "&darr;" => String::from_utf8_lossy(b"\xe2\x86\x93"),
    "&harr;" => String::from_utf8_lossy(b"\xe2\x86\x94"),
    "&crarr;" => String::from_utf8_lossy(b"\xe2\x86\xb5"),
    "&lArr;" => String::from_utf8_lossy(b"\xe2\x87\x90"),
    "&uArr;" => String::from_utf8_lossy(b"\xe2\x87\x91"),
    "&rArr;" => String::from_utf8_lossy(b"\xe2\x87\x92"),
    "&dArr;" => String::from_utf8_lossy(b"\xe2\x87\x93"),
    "&hArr;" => String::from_utf8_lossy(b"\xe2\x87\x94"),
    "&forall;" => String::from_utf8_lossy(b"\xe2\x88\x80"),
    "&part;" => String::from_utf8_lossy(b"\xe2\x88\x82"),
    "&exist;" => String::from_utf8_lossy(b"\xe2\x88\x83"),
    "&empty;" => String::from_utf8_lossy(b"\xe2\x88\x85"),
    "&nabla;" => String::from_utf8_lossy(b"\xe2\x88\x87"),
    "&isin;" => String::from_utf8_lossy(b"\xe2\x88\x88"),
    "&notin;" => String::from_utf8_lossy(b"\xe2\x88\x89"),
    "&ni;" => String::from_utf8_lossy(b"\xe2\x88\x8b"),
    "&prod;" => String::from_utf8_lossy(b"\xe2\x88\x8f"),
    "&sum;" => String::from_utf8_lossy(b"\xe2\x88\x91"),
    "&minus;" => String::from_utf8_lossy(b"\xe2\x88\x92"),
    "&lowast;" => String::from_utf8_lossy(b"\xe2\x88\x97"),
    "&radic;" => String::from_utf8_lossy(b"\xe2\x88\x9a"),
    "&prop;" => String::from_utf8_lossy(b"\xe2\x88\x9d"),
    "&infin;" => String::from_utf8_lossy(b"\xe2\x88\x9e"),
    "&ang;" => String::from_utf8_lossy(b"\xe2\x88\xa0"),
    "&and;" => String::from_utf8_lossy(b"\xe2\x88\xa7"),
    "&or;" => String::from_utf8_lossy(b"\xe2\x88\xa8"),
    "&cap;" => String::from_utf8_lossy(b"\xe2\x88\xa9"),
    "&cup;" => String::from_utf8_lossy(b"\xe2\x88\xaa"),
    "&int;" => String::from_utf8_lossy(b"\xe2\x88\xab"),
    "&there4;" => String::from_utf8_lossy(b"\xe2\x88\xb4"),
    "&sim;" => String::from_utf8_lossy(b"\xe2\x88\xbc"),
    "&cong;" => String::from_utf8_lossy(b"\xe2\x89\x85"),
    "&asymp;" => String::from_utf8_lossy(b"\xe2\x89\x88"),
    "&ne;" => String::from_utf8_lossy(b"\xe2\x89\xa0"),
    "&equiv;" => String::from_utf8_lossy(b"\xe2\x89\xa1"),
    "&le;" => String::from_utf8_lossy(b"\xe2\x89\xa4"),
    "&ge;" => String::from_utf8_lossy(b"\xe2\x89\xa5"),
    "&sub;" => String::from_utf8_lossy(b"\xe2\x8a\x82"),
    "&sup;" => String::from_utf8_lossy(b"\xe2\x8a\x83"),
    "&nsub;" => String::from_utf8_lossy(b"\xe2\x8a\x84"),
    "&sube;" => String::from_utf8_lossy(b"\xe2\x8a\x86"),
    "&supe;" => String::from_utf8_lossy(b"\xe2\x8a\x87"),
    "&oplus;" => String::from_utf8_lossy(b"\xe2\x8a\x95"),
    "&otimes;" => String::from_utf8_lossy(b"\xe2\x8a\x97"),
    "&perp;" => String::from_utf8_lossy(b"\xe2\x8a\xa5"),
    "&sdot;" => String::from_utf8_lossy(b"\xe2\x8b\x85"),
    "&lceil;" => String::from_utf8_lossy(b"\xe2\x8c\x88"),
    "&rceil;" => String::from_utf8_lossy(b"\xe2\x8c\x89"),
    "&lfloor;" => String::from_utf8_lossy(b"\xe2\x8c\x8a"),
    "&rfloor;" => String::from_utf8_lossy(b"\xe2\x8c\x8b"),
    "&lang;" => String::from_utf8_lossy(b"\xe2\x8c\xa9"),
    "&rang;" => String::from_utf8_lossy(b"\xe2\x8c\xaa"),
    "&loz;" => String::from_utf8_lossy(b"\xe2\x97\x8a"),
    "&spades;" => String::from_utf8_lossy(b"\xe2\x99\xa0"),
    "&clubs;" => String::from_utf8_lossy(b"\xe2\x99\xa3"),
    "&hearts;" => String::from_utf8_lossy(b"\xe2\x99\xa5"),
    "&diams;" => String::from_utf8_lossy(b"\xe2\x99\xa6"),
    _ => String::from_utf8_lossy(b""),
  }
  .into_owned()
}

fn escape_html(input: &str) -> String {
  input
    .chars()
    .map(|input_c| match input_c {
      '&' => "&amp;".to_string(),
      '"' => "&quot;".to_string(),
      '\'' => "apos;".to_string(),
      '<' => "&lt;".to_string(),
      '>' => "&gt;".to_string(),
      any => any.to_string(),
    })
    .fold(String::with_capacity(input.len() * 2), |work, s| {
      format!("{}{}", work, s)
    })
}

#[test]
fn escape_test() {
  let converted = escape_html("&");
  assert_eq!("&amp;".to_string(), converted);
}

fn unescape_html(input: &str) -> String {
  let mut output = String::with_capacity(input.len() + 1);
  for i in 0..input.len() {
    let input_c = input.as_bytes()[i] as char;
    if input_c != '&' {
      output.push(input_c);
      continue;
    }
    let end = input
      .get(i..)
      .expect("Invalid character sequence")
      .find(';')
      .expect("Escape sequence is not terminated with \";\"");
    let entity = input.get(i..end).unwrap();
    let code = if entity.starts_with("&#") {
      if entity.starts_with("&#x") {
        let hex = entity.trim_start_matches("&#x");
        i32::from_str_radix(hex, 16)
          .expect("Hex sequence is illegal")
          .to_string()
      } else {
        entity.trim_start_matches("&#").to_string()
      }
    } else {
      html_entity(entity)
    };
    output.push_str(code.as_str());
  }
  output
}

#[test]
fn unescape_test() {
  let converted = html_entity("&apos;");
  assert_eq!("'".to_string(), converted);
}
