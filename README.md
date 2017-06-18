# counted-rs

A client for The Guardian's database of United States police killings.

To build, clone the repository and run `cargo build`, or download from the releases.

The tool can emit either human readable or CSV output (with `-v` or `--csv`). Given no arguments (or `--all`), it fetches the entire database. To see
filters, give `--help` at the command line.

### Examples

A single filter:

```
$ ./counted --age 13
Tyre King, 13yo Black Male in Columbus, OH
```

Multiple filters:

```
$ ./counted --city "San Diego" --age 30
Joshua Sisson, 30yo White Male in San Diego, CA
Thongsoune Vilaysane, 30yo Asian/Pacific Islander Male in San Diego, CA
```

Many filters, CSV output:

```
$ ./counted --state NY --armed No --race Black -v
name,address,age,armed,cause,city,state,day,month,year,race,sex
Samuel Harrell,18 Strack Dr,30,No,Death in custody,Beacon,NY,21,April,2015,Black,Male
David Felix,538 E Sixth St,24,No,Gunshot,New York,NY,25,April,2015,Black,Male
Richard Davis,Tremont St and Morgan St,50,No,Taser,Rochester,NY,31,May,2015,Black,Male
Denzel Brown,1851 Sunrise Hwy,21,No,Gunshot,Bay Shore,NY,22,March,2015,Black,Male
Miguel Espinal,Saw Mill River Pkwy,36,No,Gunshot,Yonkers,NY,8,December,2015,Black,Male
Donald 'Dontay' Ivy,Lark St and Second St,39,No,Taser,Albany,NY,2,April,2015,Black,Male
Felix Kumi,Beekman Ave and Tecumseh Ave,61,No,Gunshot,Mount Vernon,NY,28,August,2015,Black,Male
Delrawn Small,Atlantic Ave,37,No,Gunshot,Brooklyn,NY,4,July,2016,Black,Male
```
