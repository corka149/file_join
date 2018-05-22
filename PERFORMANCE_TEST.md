# Performance test

This is mainly for orientating and documentation of performance of the single-thread version 
vs the multi-thread version of file_join.

## Hardware
Tests executed on the following hardware:
```
    Beschreibung: Notebook
    Produkt: GP62 2QE 
     *-cpu
          Beschreibung: CPU
          Produkt: Intel(R) Core(TM) i7-5700HQ CPU @ 2.70GHz
          Hersteller: Intel Corp.
          Physische ID: 41
          Bus-Informationen: cpu@0
          Version: Intel(R) Core(TM) i7-5700HQ CPU @ 2.70GHz
          Seriennummer: NULL
          Steckplatz: SOCKET 0
          Größe: 2362MHz
          Kapazität: 3500MHz
          Breite: 64 bits
          Takt: 100MHz
          Konfiguration: cores=4 enabledcores=4 threads=8

     *-memory
          Beschreibung: Systemspeicher
          Physische ID: 43
          Steckplatz: Systemplatine oder Hauptplatine
          Größe: 8GiB
        *-bank:0
             Beschreibung: SODIMM DDR3 Synchron 1600 MHz (0,6 ns)
             Produkt: HMT41GS6BFR8A-PB
             Hersteller: Hynix/Hyundai
             Physische ID: 0
             Seriennummer: 1C745ADC
             Steckplatz: ChannelA-DIMM0
             Größe: 8GiB
             Breite: 64 bits
             Takt: 1600MHz (0.6ns)

```

## Operating System and software
Tests were executed on the following operating system:
```
$ uname -a
 Linux corka-GP62-2QE 4.15.0-20-generic #21-Ubuntu SMP Tue Apr 24 06:16:15 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux
$ cat /etc/*-release
 DISTRIB_ID=Ubuntu
 DISTRIB_RELEASE=18.04
 DISTRIB_CODENAME=bionic
 DISTRIB_DESCRIPTION="Ubuntu 18.04 LTS"
 NAME="Ubuntu"
 VERSION="18.04 LTS (Bionic Beaver)"
 ID=ubuntu
 ID_LIKE=debian
 PRETTY_NAME="Ubuntu 18.04 LTS"
 VERSION_ID="18.04"
 VERSION_CODENAME=bionic
 UBUNTU_CODENAME=bionic

```
To clock the time the "time" within bash was used. [meaning of time-result](https://stackoverflow.com/questions/556405/what-do-real-user-and-sys-mean-in-the-output-of-time1/556411#556411)

## Test data

Test with 10 books from:
- http://gutenberg.ca/ebooks/footnerh-whokilledthehusband/footnerh-whokilledthehusband-00-t.txt
- http://gutenberg.ca/ebooks/flemingi-onhermajestyssecretservice/flemingi-onhermajestyssecretservice-00-t.txt
- http://gutenberg.ca/ebooks/bottomep-windlestraws/bottomep-windlestraws-00-t.txt
- http://gutenberg.ca/ebooks/farleyrm-liquidlife/farleyrm-liquidlife-00-t.txt
- http://gutenberg.ca/ebooks/biggersed-housewithoutakey/biggersed-housewithoutakey-00-t.txt

in a subfolder "testest"
- http://gutenberg.ca/ebooks/farleyrm-goldencity/farleyrm-goldencity-00-t.txt
- http://gutenberg.ca/ebooks/forestercs-thegeneral/forestercs-thegeneral-00-t.txt
- http://gutenberg.ca/ebooks/freemanra-asathiefinthenight/freemanra-asathiefinthenight-00-t.txt
- http://gutenberg.ca/ebooks/frankp-mradam/frankp-mradam-00-t.txt
- http://gutenberg.ca/ebooks/grahamg-earthandhighheaven/grahamg-earthandhighheaven-00-t.txt

## Stopped tests - Lesson learned

It turns out that this amount of data is not a challenge. Hence I doubled the content of each file until the complete
test folder reaches a size of one gigabyte. In this case the I/O bandwidth is the limit. I doubt that multithreading is
the solution so I will drop the idea of using multithreading.