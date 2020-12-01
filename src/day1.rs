
fn check_for_sum_and_multiply(list: Vec<i32>, sum: i32) -> i32 {
    for s1 in &list {
        for s2 in &list {
            if s1 + s2 == sum {
                return s1 * s2;
            }
        }
    }

    return 0;
}

fn check_for_triple_sum_and_multiply(list: Vec<i32>, sum: i32) -> i32 {
    for s1 in &list {
        for s2 in &list {
            for s3 in &list {
                if s1 + s2 + s3 == sum {
                    return s1 * s2 * s3;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day1::check_for_sum_and_multiply;
    use crate::day1::check_for_triple_sum_and_multiply;

    #[test]
    fn example() {
        let the_list = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(check_for_sum_and_multiply(the_list, 2020), 514579);
    }

    #[test]
    fn example2() {
        let the_list = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(check_for_triple_sum_and_multiply(the_list, 2020), 241861950);
    }

    #[test]
    fn first_star() {
        let the_list = vec![
            1429, 1368, 1661, 1687, 1593, 1495, 1565, 1500, 1635, 1845, 1645, 1999, 1415, 1054, 1930, 1774,
            1405, 1993, 1757, 1623, 1675, 1665, 631, 1950, 1702, 1311, 1509, 1790, 1643, 1884, 226, 1455,
            1679, 1746, 1284, 1342, 1684, 1543, 1396, 1806, 1523, 1363, 1011, 1577, 1767, 1287, 1885,
            1517,
            1556,
            1722,
            1260,
            1624,
            1466,
            1263,
            1162,
            1688,
            1202,
            1913,
            1964,
            1385,
            1970,
            1976,
            1431,
            858,
            1748,
            1544,
            1438,
            1300,
            1926,
            1587,
            1376,
            1939,
            1039,
            1639,
            1539,
            1491,
            1631,
            1521,
            1564,
            1507,
            1637,
            1534,
            1713,
            1533,
            1118,
            1356,
            2003,
            282,
            1079,
            1837,
            1259,
            1941,
            1836,
            1903,
            1433,
            1467,
            1027,
            1441,
            1048,
            1742,
            1087,
            1872,
            1476,
            1657,
            1361,
            1182,
            1494,
            1529,
            1822,
            1444,
            1330,
            1514,
            1723,
            1432,
            1683,
            1997,
            1443,
            1474,
            1932,
            1504,
            1313,
            1765,
            19,
            1784,
            1619,
            992,
            1560,
            1680,
            1626,
            1558,
            1899,
            1293,
            1676,
            1161,
            1140,
            1341,
            1597,
            1628,
            1611,
            1302,
            1269,
            1241,
            1952,
            1591,
            1726,
            428,
            1703,
            1289,
            1109,
            1478,
            1002,
            1817,
            1849,
            1838,
            1319,
            1641,
            583,
            1920,
            1453,
            1411,
            1870,
            1763,
            1469,
            1646,
            1719,
            1213,
            1462,
            1545,
            1682,
            1711,
            18,
            2004,
            1252,
            1620,
            1559,
            1315,
            781,
            1656,
            1987,
            1436,
            1630,
            1985,
            1897,
            1551,
            1296,
            1282,
            1735,
            1320,
            1659,
            1271,
            1380,
            1274,
            1876,
            1492,
            1298,
            1399,
            1692,
            1265,
            1555,
            1337];

        let res = check_for_sum_and_multiply(the_list, 2020);

        println!("Result is {:?}", res);
        assert_eq!(res, 996996);
    }

    #[test]
    fn extra() {
        let the_list = vec![
            1429, 1368, 1661, 1687, 1593, 1495, 1565, 1500, 1635, 1845, 1645, 1999, 1415, 1054, 1930, 1774,
            1405, 1993, 1757, 1623, 1675, 1665, 631, 1950, 1702, 1311, 1509, 1790, 1643, 1884, 226, 1455,
            1679, 1746, 1284, 1342, 1684, 1543, 1396, 1806, 1523, 1363, 1011, 1577, 1767, 1287, 1885,
            1517,
            1556,
            1722,
            1260,
            1624,
            1466,
            1263,
            1162,
            1688,
            1202,
            1913,
            1964,
            1385,
            1970,
            1976,
            1431,
            858,
            1748,
            1544,
            1438,
            1300,
            1926,
            1587,
            1376,
            1939,
            1039,
            1639,
            1539,
            1491,
            1631,
            1521,
            1564,
            1507,
            1637,
            1534,
            1713,
            1533,
            1118,
            1356,
            2003,
            282,
            1079,
            1837,
            1259,
            1941,
            1836,
            1903,
            1433,
            1467,
            1027,
            1441,
            1048,
            1742,
            1087,
            1872,
            1476,
            1657,
            1361,
            1182,
            1494,
            1529,
            1822,
            1444,
            1330,
            1514,
            1723,
            1432,
            1683,
            1997,
            1443,
            1474,
            1932,
            1504,
            1313,
            1765,
            19,
            1784,
            1619,
            992,
            1560,
            1680,
            1626,
            1558,
            1899,
            1293,
            1676,
            1161,
            1140,
            1341,
            1597,
            1628,
            1611,
            1302,
            1269,
            1241,
            1952,
            1591,
            1726,
            428,
            1703,
            1289,
            1109,
            1478,
            1002,
            1817,
            1849,
            1838,
            1319,
            1641,
            583,
            1920,
            1453,
            1411,
            1870,
            1763,
            1469,
            1646,
            1719,
            1213,
            1462,
            1545,
            1682,
            1711,
            18,
            2004,
            1252,
            1620,
            1559,
            1315,
            781,
            1656,
            1987,
            1436,
            1630,
            1985,
            1897,
            1551,
            1296,
            1282,
            1735,
            1320,
            1659,
            1271,
            1380,
            1274,
            1876,
            1492,
            1298,
            1399,
            1692,
            1265,
            1555,
            1337];

        let res = check_for_triple_sum_and_multiply(the_list, 2020);

        println!("Result is {:?}", res);
        assert_eq!(res, 9210402);
    }
}