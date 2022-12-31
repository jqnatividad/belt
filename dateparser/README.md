# qsv-dateparser

A rust library for parsing date strings in commonly used formats. Parsed date will be returned as `chrono`'s
`DateTime<Utc>`.

This is a fork of Rollie Ma's [dateparser](https://github.com/waltzofpearls/belt/tree/main/dateparser), specifically published to support [qsv](https://github.com/jqnatividad/qsv).
It supports a subset of date formats supported by dateparser, skipping more obscure formats, primarily
for performance.
It also adds support for parsing dates in DMY format, with the `parse_with_preference` function.

It will also be the foundation for replacing the chrono crate, with its [long-standing security advisory](https://rustsec.org/advisories/RUSTSEC-2020-0159).

## Accepted date formats
```rust
// unix timestamp
"1511648546",
"1620021848429",
"1620024872717915000",
"0",
"-770172300",
"1671673426.123456789",
// rfc3339
"2021-05-01T01:17:02.604456Z",
"2017-11-25T22:34:50Z",
// rfc2822
"Wed, 02 Jun 2021 06:31:39 GMT",
// yyyy-mm-dd hh:mm:ss
"2014-04-26 05:24:37 PM",
"2021-04-30 21:14",
"2021-04-30 21:14:10",
"2021-04-30 21:14:10.052282",
"2014-04-26 17:24:37.123",
"2014-04-26 17:24:37.3186369",
"2012-08-03 18:31:59.257000000",
// yyyy-mm-dd hh:mm:ss z
"2017-11-25 13:31:15 PST",
"2017-11-25 13:31 PST",
"2014-12-16 06:20:00 UTC",
"2014-12-16 06:20:00 GMT",
"2014-04-26 13:13:43 +0800",
"2014-04-26 13:13:44 +09:00",
"2012-08-03 18:31:59.257000000 +0000",
"2015-09-30 18:48:56.35272715 UTC",
// yyyy-mm-dd
"2021-02-21",
// yyyy-mm-dd z
"2021-02-21 PST",
"2021-02-21 UTC",
"2020-07-20+08:00",
// Mon dd, yyyy, hh:mm:ss
"May 8, 2009 5:57:51 PM",
"September 17, 2012 10:09am",
"September 17, 2012, 10:10:09",
// Mon dd, yyyy hh:mm:ss z
"May 02, 2021 15:51:31 UTC",
"May 02, 2021 15:51 UTC",
"May 26, 2021, 12:49 AM PDT",
"September 17, 2012 at 10:09am PST",
// yyyy-mon-dd
"2021-Feb-21",
// Mon dd, yyyy
"May 25, 2021",
"oct 7, 1970",
"oct 7, 70",
"oct. 7, 1970",
"oct. 7, 70",
"October 7, 1970",
// dd Mon yyyy hh:mm:ss
"12 Feb 2006, 19:17",
"12 Feb 2006 19:17",
"14 May 2019 19:11:40.164",
// dd Mon yyyy
"7 oct 70",
"7 oct 1970",
"03 February 2013",
"1 July 2013",
// mm/dd/yyyy hh:mm:ss
"4/8/2014 22:05",
"04/08/2014 22:05",
"4/8/14 22:05",
"04/2/2014 03:00:51",
"8/8/1965 12:00:00 AM",
"8/8/1965 01:00:01 PM",
"8/8/1965 01:00 PM",
"8/8/1965 1:00 PM",
"8/8/1965 12:00 AM",
"4/02/2014 03:00:51",
"03/19/2012 10:11:59",
"03/19/2012 10:11:59.3186369",
// mm/dd/yyyy
"3/31/2014",
"03/31/2014",
"08/21/71",
"8/1/71",
// yyyy/mm/dd hh:mm:ss
"2014/4/8 22:05",
"2014/04/08 22:05",
"2014/04/2 03:00:51",
"2014/4/02 03:00:51",
"2012/03/19 10:11:59",
"2012/03/19 10:11:59.3186369",
// yyyy/mm/dd
"2014/3/31",
"2014/03/31",
// dd/mm/yyyy
"31/12/2020",
"12/10/2019",
"03/06/2018",
"27/06/68",
// dd/mm/yyyy hh:mm:ss
"4/8/2014 22:05",
"04/08/2014 22:05",
"4/8/14 22:05",
"04/2/2014 03:00:51",
"8/8/1965 12:00:00 AM",
"8/8/1965 01:00:01 PM",
"8/8/1965 01:00 PM",
"31/12/22 15:00"
```
