// AUTOGENERATED FROM index-windows-1251.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1251.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    1026, 1027, 8218, 1107, 8222, 8230, 8224, 8225, 8364, 8240, 1033, 8249,
    1034, 1036, 1035, 1039, 1106, 8216, 8217, 8220, 8221, 8226, 8211, 8212,
    152, 8482, 1113, 8250, 1114, 1116, 1115, 1119, 160, 1038, 1118, 1032, 164,
    1168, 166, 167, 1025, 169, 1028, 171, 172, 173, 174, 1031, 176, 177, 1030,
    1110, 1169, 181, 182, 183, 1105, 8470, 1108, 187, 1112, 1029, 1109, 1111,
    1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063,
    1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075,
    1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087,
    1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099,
    1100, 1101, 1102, 1103,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        1026 => 128, 1027 => 129, 8218 => 130, 1107 => 131, 8222 => 132,
        8230 => 133, 8224 => 134, 8225 => 135, 8364 => 136, 8240 => 137,
        1033 => 138, 8249 => 139, 1034 => 140, 1036 => 141, 1035 => 142,
        1039 => 143, 1106 => 144, 8216 => 145, 8217 => 146, 8220 => 147,
        8221 => 148, 8226 => 149, 8211 => 150, 8212 => 151, 152 => 152,
        8482 => 153, 1113 => 154, 8250 => 155, 1114 => 156, 1116 => 157,
        1115 => 158, 1119 => 159, 160 => 160, 1038 => 161, 1118 => 162,
        1032 => 163, 164 => 164, 1168 => 165, 166 => 166, 167 => 167,
        1025 => 168, 169 => 169, 1028 => 170, 171 => 171, 172 => 172,
        173 => 173, 174 => 174, 1031 => 175, 176 => 176, 177 => 177,
        1030 => 178, 1110 => 179, 1169 => 180, 181 => 181, 182 => 182,
        183 => 183, 1105 => 184, 8470 => 185, 1108 => 186, 187 => 187,
        1112 => 188, 1029 => 189, 1109 => 190, 1111 => 191, 1040 => 192,
        1041 => 193, 1042 => 194, 1043 => 195, 1044 => 196, 1045 => 197,
        1046 => 198, 1047 => 199, 1048 => 200, 1049 => 201, 1050 => 202,
        1051 => 203, 1052 => 204, 1053 => 205, 1054 => 206, 1055 => 207,
        1056 => 208, 1057 => 209, 1058 => 210, 1059 => 211, 1060 => 212,
        1061 => 213, 1062 => 214, 1063 => 215, 1064 => 216, 1065 => 217,
        1066 => 218, 1067 => 219, 1068 => 220, 1069 => 221, 1070 => 222,
        1071 => 223, 1072 => 224, 1073 => 225, 1074 => 226, 1075 => 227,
        1076 => 228, 1077 => 229, 1078 => 230, 1079 => 231, 1080 => 232,
        1081 => 233, 1082 => 234, 1083 => 235, 1084 => 236, 1085 => 237,
        1086 => 238, 1087 => 239, 1088 => 240, 1089 => 241, 1090 => 242,
        1091 => 243, 1092 => 244, 1093 => 245, 1094 => 246, 1095 => 247,
        1096 => 248, 1097 => 249, 1098 => 250, 1099 => 251, 1100 => 252,
        1101 => 253, 1102 => 254, 1103 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}