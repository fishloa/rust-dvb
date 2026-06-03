ETSI EN 302 307-1
V1.4.1
(2014-11)
EUROPEAN STANDARD
Digital Video Broadcasting (DVB);
Second generation framing structure, channel coding and
modulation systems for Broadcasting, Interactive Services,
News Gathering and other broadband satellite applications;
Part 1: DVB-S2

2 ETSI EN 302 307-1 V1.4.1 (2014-11)
Reference
REN/JTC-DVB-341-1
Keywords
BSS, digital, DVB, modulation, satellite, TV
ETSI
650 Route des Lucioles
F-06921 Sophia Antipolis Cedex - FRANCE
Tel.: +33 4 92 94 42 00 Fax: +33 4 93 65 47 16
Siret N° 348 623 562 00017 - NAF 742 C
Association à but non lucratif enregistrée à la
Sous-Préfecture de Grasse (06) N° 7803/88
Important notice
The present document can be downloaded from:
http://www.etsi.org
The present document may be made available in electronic versions and/or in print. The content of any electronic and/or
print versions of the present document shall not be modified without the prior written authorization of ETSI. In case of any
existing or perceived difference in contents between such versions and/or in print, the only prevailing document is the
print of the Portable Document Format (PDF) version kept on a specific network drive within ETSI Secretariat.
Users of the present document should be aware that the document may be subject to revision or change of status.
Information on the current status of this and other ETSI documents is available at
http://portal.etsi.org/tb/status/status.asp
If you find errors in the present document, please send your comment to one of the following services:
http://portal.etsi.org/chaircor/ETSI_support.asp
Copyright Notification
No part may be reproduced or utilized in any form or by any means, electronic or mechanical, including photocopying
and microfilm except as authorized by written permission of ETSI.
The content of the PDF version shall not be modified without the written authorization of ETSI.
The copyright and the foregoing restriction extend to reproduction in all media.
© European Telecommunications Standards Institute 2014.
© European Broadcasting Union 2014.
All rights reserved.
DECTTM, PLUGTESTSTM, UMTSTM and the ETSI logo are Trade Marks of ETSI registered for the benefit of its Members.
3GPPTM and LTE™ are Trade Marks of ETSI registered for the benefit of its Members and
of the 3GPP Organizational Partners.
GSM® and the GSM logo are Trade Marks registered and owned by the GSM Association.
ETSI

3 ETSI EN 302 307-1 V1.4.1 (2014-11)
Contents
Intellectual Property Rights ................................................................................................................................ 6
Foreword ............................................................................................................................................................. 6
Modal verbs terminology .................................................................................................................................... 7
Introduction ........................................................................................................................................................ 7
1 Scope ........................................................................................................................................................ 9
2 References ................................................................................................................................................ 9
2.1 Normative references ......................................................................................................................................... 9
2.2 Informative references ...................................................................................................................................... 10
3 Symbols and abbreviations ..................................................................................................................... 10
3.1 Symbols ............................................................................................................................................................ 10
3.2 Abbreviations ................................................................................................................................................... 11
4 Transmission system description ............................................................................................................ 13
4.1 System definition .............................................................................................................................................. 13
4.2 System architecture .......................................................................................................................................... 14
4.3 System configurations ...................................................................................................................................... 15
5 Subsystems specification ........................................................................................................................ 16
5.1 Mode adaptation ............................................................................................................................................... 16
5.1.1 Input interface ............................................................................................................................................. 17
5.1.2 Input stream synchronizer (optional, not relevant for single TS - BS) ....................................................... 17
5.1.3 Null-Packet Deletion (ACM and Transport Stream only) .......................................................................... 17
5.1.4 CRC-8 encoder (for packetized streams only) ............................................................................................ 18
5.1.5 Merger/Slicer .............................................................................................................................................. 18
5.1.6 Base-Band Header insertion ....................................................................................................................... 19
5.2 Stream adaptation ............................................................................................................................................. 21
5.2.1 Padding ....................................................................................................................................................... 21
5.2.2 BB scrambling ............................................................................................................................................ 21
5.3 FEC encoding ................................................................................................................................................... 22
5.3.1 Outer encoding (BCH) ................................................................................................................................ 23
5.3.2 Inner encoding (LDPC) .............................................................................................................................. 24
5.3.2.1 Inner coding for normal FECFRAME................................................................................................... 24
5.3.2.2 Inner coding for short FECFRAME ...................................................................................................... 25
5.3.3 Bit Interleaver (for 8PSK, 16APSK and 32APSK only) ............................................................................. 26
5.4 Bit mapping into constellation.......................................................................................................................... 27
5.4.1 Bit mapping into QPSK constellation ......................................................................................................... 27
5.4.2 Bit mapping into 8PSK constellation .......................................................................................................... 28
5.4.3 Bit mapping into 16APSK constellation ..................................................................................................... 28
5.4.4 Bit mapping into 32APSK .......................................................................................................................... 29
5.5 Physical Layer (PL) framing ............................................................................................................................ 30
5.5.1 Dummy PLFRAME insertion ..................................................................................................................... 31
5.5.2 PL signalling ............................................................................................................................................... 31
5.5.2.1 SOF field ............................................................................................................................................... 32
5.5.2.2 MODCOD field ..................................................................................................................................... 32
5.5.2.3 TYPE field ............................................................................................................................................ 32
5.5.2.4 PLS code ............................................................................................................................................... 32
5.5.3 Pilots insertion ............................................................................................................................................ 33
5.5.4 Physical layer scrambling ........................................................................................................................... 33
5.6 Baseband shaping and quadrature modulation ................................................................................................. 35
6 Error performance .................................................................................................................................. 36
Annex A (normative): Signal spectrum at the modulator output .................................................... 37
Annex B (normative): Addresses of parity bit accumulators for nldpc = 64 800 ........................... 39
ETSI

4 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex C (normative): Addresses of parity bit accumulators for nldpc = 16 200 ........................... 51
Annex D (normative): Additional Mode Adaptation and ACM tools ............................................. 58
D.1 "ACM Command" signalling interface .................................................................................................. 58
D.2 Input stream synchronizer ...................................................................................................................... 58
D.3 Null-packet Deletion (normative for input transport streams and ACM)............................................... 60
D.4 BBHEADER and Merging/slicing Policy for various application areas ................................................ 61
D.5 Signalling of reception quality via return channel (Normative for ACM) ............................................. 62
Annex E (normative): SI and signal identification for DSNG and contribution applications ...... 64
Annex F (normative): Backwards Compatible modes (optional) .................................................... 65
Annex G (informative): Supplementary information on receiver implementation .......................... 66
G.1 Carrier recovery ...................................................................................................................................... 66
G.2 FEC decoding ......................................................................................................................................... 66
G.3 ACM: Transport Stream regeneration and clock recovery using ISCR ................................................. 66
G.4 Non linearity pre-compensation and Intersymbol Interference suppression techniques ........................ 66
G.5 Interactive services using DVB-RCS return link: user terminal synchronization .................................. 67
Annex H (informative): Examples of possible use of the System ........................................................ 68
H.1 CCM digital TV broadcasting: bit rate capacity and C/N requirements ................................................ 68
H.2 Distribution of multiple TS multiplexes to DTT Transmitters (Multiple TS, CCM) ............................. 68
H.3 SDTV and HDTV broadcasting with differentiated protection (VCM, Multiple TS) ........................... 68
H.4 DSNG Services using ACM (Single transport Stream, information rate varying in time) .................... 68
H.5 IP Unicast Services (Non-uniform protection on a user-by-user basis) ................................................. 68
H.6 Example performance of BC modes....................................................................................................... 68
H.7 Satellite transponder models for simulations ......................................................................................... 68
H.8 Phase noise masks for simulations ......................................................................................................... 71
Annex I (normative): Mode Adaptation input interfaces (optional) .............................................. 72
I.1 Mode Adaptation input interface with separate signalling circuit (optional) ......................................... 72
I.2 Mode Adaptation input interface with in-band signalling (optional) ..................................................... 73
Annex J (informative): Bibliography ................................................................................................... 74
Annex K: For future use ........................................................................................................................ 75
Annex L: For future use ........................................................................................................................ 76
Annex M (normative): Transmission format for wideband satellite transponders using time-
slicing (optional) ............................................................................................. 77
M.1 Definition of Time-slicing receiver ........................................................................................................ 77
M.2 TIME SLICE MODE CODING ............................................................................................................. 78
M.2.1 PL signalling .................................................................................................................................................... 78
M.2.2 SOF field .......................................................................................................................................................... 79
M.2.3 MODCOD field ................................................................................................................................................ 79
M.2.4 TYPE field........................................................................................................................................................ 79
M.2.5 TSN code .......................................................................................................................................................... 79
ETSI

|     | 5   | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
M.3  Phase noise masks .................................................................................................................................. 79
History .............................................................................................................................................................. 80

ETSI

6 ETSI EN 302 307-1 V1.4.1 (2014-11)
Intellectual Property Rights
IPRs essential or potentially essential to the present document may have been declared to ETSI. The information
pertaining to these essential IPRs, if any, is publicly available for ETSI members and non-members, and can be found
in ETSI SR 000 314: "Intellectual Property Rights (IPRs); Essential, or potentially Essential, IPRs notified to ETSI in
respect of ETSI standards", which is available from the ETSI Secretariat. Latest updates are available on the ETSI Web
server (http://ipr.etsi.org).
Pursuant to the ETSI IPR Policy, no investigation, including IPR searches, has been carried out by ETSI. No guarantee
can be given as to the existence of other IPRs not referenced in ETSI SR 000 314 (or the updates on the ETSI Web
server) which are, or may be, or may become, essential to the present document.
Foreword
This European Standard (EN) has been produced by Joint Technical Committee (JTC) Broadcast of the European
Broadcasting Union (EBU), Comité Européen de Normalisation ELECtrotechnique (CENELEC) and the European
Telecommunications Standards Institute (ETSI).
NOTE: The EBU/ETSI JTC Broadcast was established in 1990 to co-ordinate the drafting of standards in the
specific field of broadcasting and related fields. Since 1995 the JTC Broadcast became a tripartite body
by including in the Memorandum of Understanding also CENELEC, which is responsible for the
standardization of radio and television receivers. The EBU is a professional association of broadcasting
organizations whose work includes the co-ordination of its members' activities in the technical, legal,
programme-making and programme-exchange domains. The EBU has active members in about
60 countries in the European broadcasting area; its headquarters is in Geneva.
European Broadcasting Union
CH-1218 GRAND SACONNEX (Geneva)
Switzerland
Tel: +41 22 717 21 11
Fax: +41 22 717 24 81
The Digital Video Broadcasting Project (DVB) is an industry-led consortium of broadcasters, manufacturers, network
operators, software developers, regulatory bodies, content owners and others committed to designing global standards
for the delivery of digital television and data services. DVB fosters market driven solutions that meet the needs and
economic circumstances of broadcast industry stakeholders and consumers. DVB standards cover all aspects of digital
television from transmission through interfacing, conditional access and interactivity for digital video, audio and data.
The consortium came together in 1993 to provide global standardization, interoperability and future proof
specifications.
The present document is part 1 of a multi-part deliverable covering a "second generation" modulation and channel
coding system, denoted "DVB-S2", as identified below:
Part 1: "DVB-S2";
Part 2: "DVB-S2-Extensions (DVB-S2X)".
National transposition dates
Date of adoption of this EN: 4 November 2014
Date of latest announcement of this EN (doa): 28 February 2015
Date of latest publication of new National Standard
or endorsement of this EN (dop/e): 31 August 2015
Date of withdrawal of any conflicting National Standard (dow): 31 August 2015
ETSI

7 ETSI EN 302 307-1 V1.4.1 (2014-11)
Modal verbs terminology
In the present document "shall", "shall not", "should", "should not", "may", "may not", "need", "need not", "will",
"will not", "can" and "cannot" are to be interpreted as described in clause 3.2 of the ETSI Drafting Rules (Verbal forms
for the expression of provisions).
"must" and "must not" are NOT allowed in ETSI deliverables except when used in direct citation.
Introduction
DVB-S (EN 300 421 [2]) was introduced as a standard in 1994 and DVB-DSNG (EN 301 210 [3]) in 1997. The DVB-S
standard specifies QPSK modulation and concatenated convolutional and Reed-Solomon channel coding, and is now
used by most satellite operators worldwide for television and data broadcasting services. DVB-DSNG specifies, in
addition to DVB-S format, the use of 8PSK and 16QAM modulation for satellite news gathering and contribution
services.
Since 1997, digital satellite transmission technology has evolved somewhat:
• New channel coding schemes, combined with higher order modulation, promise more powerful alternatives to
the DVB-S/DVB-DSNG coding and modulation schemes. The result is a capacity gain in the order of 30 % at
a given transponder bandwidth and transmitted EIRP, depending on the modulation type and code rate.
• Variable Coding and Modulation (VCM) may be applied to provide different levels of error protection to
different service components (e.g. SDTV and HDTV, audio, multimedia).
• In the case of interactive and point-to-point applications, the VCM functionality may be combined with the use
of return channels, to achieve Adaptive Coding and Modulation (ACM). This technique provides more exact
channel protection and dynamic link adaptation to propagation conditions, targeting each individual receiving
terminal. ACM systems promise satellite capacity gains of up to 100 % to 200 %. In addition, service
availability may be extended compared to a constant protection system (CCM) such as DVB-S or
DVB-DSNG. Such gains are achieved by informing the satellite up-link station of the channel condition
(e.g. C/N+I) of each receiving terminal via the satellite or terrestrial return channels.
• DVB-S and DVB-DSNG are strictly focused on a unique data format, the MPEG Transport Stream
(ISO/IEC 13818-1 [1] or a reference to it). Extended flexibility to cope with other input data formats (such as
multiple Transport Streams, or generic data formats) is now possible without significant complexity increase.
The present document defines a "second generation" modulation and channel coding system (denoted the "System" or
"DVB-S2" for the purposes of the present document) to make use of the improvements listed above. DVB-S2 is a
single, very flexible standard, covering a variety of applications by satellite, as described below. It is characterized by:
• a flexible input stream adapter, suitable for operation with single and multiple input streams of various formats
(packetized or continuous);
• a powerful FEC system based on LDPC (Low-Density Parity Check) codes concatenated with BCH codes,
allowing Quasi-Error-Free operation at about 0,7 dB to 1 dB from the Shannon limit, depending on the
transmission mode (AWGN channel, modulation constrained Shannon limit);
• a wide range of code rates (from 1/4 up to 9/10); 4 constellations, ranging in spectrum efficiency from
2 bit/s/Hz to 5 bit/s/Hz, optimized for operation over non-linear transponders;
• a set of three spectrum shapes with roll-off factors 0,35, 0,25 and 0,20;
• Adaptive Coding and Modulation (ACM) functionality, optimizing channel coding and modulation on a
frame-by-frame basis.
The System has been optimized for the following broadband satellite applications:
Broadcast Services (BS) Digital multi-programme Television (TV)/High Definition Television (HDTV)
Broadcasting services to be used for primary and secondary distribution in the Fixed Satellite Service (FSS) and the
Broadcast Satellite Service (BSS) bands.
ETSI

8 ETSI EN 302 307-1 V1.4.1 (2014-11)
DVB-S2 is intended to provide Direct-To-Home (DTH) services for consumer Integrated Receiver Decoder (IRD), as
well as collective antenna systems (Satellite Master Antenna Television - SMATV) and cable television head-end
stations (possibly with remodulation, see EN 300 429 [5]). DVB-S2 may be considered a successor to the current
DVB-S standard EN 300 421 [2], and may be introduced for new services and allow for a long-term migration. BS
services are transported in MPEG Transport Stream format. VCM may be applied on multiple transport stream to
achieve a differentiated error protection for different services (TV, HDTV, audio, multimedia).
Interactive Services (IS) Interactive data services including Internet access
DVB-S2 is intended to provide interactive services to consumer IRDs and to personal computers, where DVB-S2's
forward path supersedes the current DVB-S standard EN 300 421 [2] for interactive systems. The return path can be
implemented using various DVB interactive systems, such as DVB-RCS (EN 301 790 [6]), DVB-RCP
(ETS 300 801 [7]), DVB-RCG (EN 301 195 [8]), DVB-RCC (ES 200 800 [9]). Data services are transported in (single
or multiple) Transport Stream format according to EN 301 192 [4] (e.g. using Multiprotocol Encapsulation), or in
(single or multiple) generic stream format. DVB-S2 can provide Constant Coding and Modulation (CCM), or Adaptive
Coding and Modulation (ACM), where each individual satellite receiving station controls the protection mode of the
traffic addressed to it. Input Stream Adaptation for ACM is specified in annex D.
Digital TV Contribution and Satellite News Gathering (DTVC/DSNG)
Digital television contribution applications by satellite consist of point-to-point or point-to-multipoint transmissions,
connecting fixed or transportable uplink and receiving stations. They are not intended for reception by the general
public. According to Recommendation ITU-R SNG.770-1 [10], SNG is defined as "Temporary and occasional
transmission with short notice of television or sound for broadcasting purposes, using highly portable or transportable
uplink earth stations ...". Services are transported in single (or multiple) MPEG Transport Stream format. DVB-S2 can
provide Constant Coding and Modulation (CCM), or Adaptive Coding and Modulation (ACM). In this latter case, a
single satellite receiving station typically controls the protection mode of the full multiplex. Input Stream Adaptation for
ACM is specified in annex D.
Data content distribution/trunking and other professional applications (PS)
These services are mainly point-to-point or point-to-multipoint, including interactive services to professional head-ends,
which re-distribute services over other media. Services may be transported in (single or multiple) generic stream format.
The system can provide Constant Coding and Modulation (CCM), Variable Coding and Modulation (VCM) or
Adaptive Coding and Modulation (ACM). In this latter case, a single satellite receiving station typically controls the
protection mode of the full TDM multiplex, or multiple receiving stations control the protection mode of the traffic
addressed to each one. In either case, interactive or non-interactive, the present document is only concerned with the
forward broadband channel.
DVB-S2 is suitable for use on different satellite transponder bandwidths and frequency bands. The symbol rate is
matched to given transponder characteristics, and, in the case of multiple carriers per transponder (FDM), to the
frequency plan adopted. Examples of possible DVB-S2 use are given in TR 102 376 [i.5].
Annex M specifies the implementation of a DVB-S2 profile suitable for operation in wide-band mode, without
requiring a full-speed decoding of the total carrier capacity, by suitably mapping the transmitted services in time-slices.
Digital transmissions via satellite are affected by power and bandwidth limitations. Therefore DVB-S2 provides for
many transmission modes (FEC coding and modulations), giving different trade-offs between power and spectrum
efficiency (see TR 102 376 [i.5]). For some specific applications (e.g. broadcasting) modes such as QPSK and 8PSK,
with their quasi-constant envelope, are appropriate for operation with saturated satellite power amplifiers (in single
carrier per transponder configuration). When higher power margins are available, spectrum efficiency can be further
increased to reduce bit delivery cost. In these cases also 16APSK and 32APSK can operate in single carrier mode close
to the satellite HPA saturation by pre-distortion techniques. All the modes are appropriate for operation in quasi-linear
satellite channels, in multi-carrier Frequency Division Multiplex (FDM) type applications.
DVB-S2 is compatible with Moving Pictures Experts Group (MPEG-2 and MPEG-4) coded TV services (see
ISO/IEC 13818-1 [1]), with a Transport Stream packet multiplex. Multiplex flexibility allows the use of the
transmission capacity for a variety of TV service configurations, including sound and data services. All service
components are Time Division Multiplexed (TDM) on a single digital carrier.
ETSI

9 ETSI EN 302 307-1 V1.4.1 (2014-11)
1 Scope
The present document:
• gives a general description of the DVB-S2 system;
• specifies the digitally modulated signal in order to allow compatibility between pieces of equipment developed
by different manufacturers. This is achieved by describing in detail the signal processing principles at the
modulator side, while the processing at the receive side is left open to different implementation solutions.
However, it is necessary in the present document to refer to certain aspects of reception;
• identifies the global performance requirements and features of the System, in order to meet the service quality
targets.
2 References
References are either specific (identified by date of publication and/or edition number or version number) or
non-specific. For specific references, only the cited version applies. For non-specific references, the latest version of the
reference document (including any amendments) applies.
Referenced documents which are not found to be publicly available in the expected location might be found at
http://docbox.etsi.org/Reference.
NOTE: While any hyperlinks included in this clause were valid at the time of publication, ETSI cannot guarantee
their long term validity.
2.1 Normative references
The following referenced documents are necessary for the application of the present document.
[1] ISO/IEC 13818 (parts 1 and 2): "Information technology -- Generic coding of moving pictures and
associated audio information".
[2] ETSI EN 300 421 (V.1.1.2): "Digital Video Broadcasting (DVB); Framing structure, channel
coding and modulation for 11/12 GHz satellite services".
[3] ETSI EN 301 210: "Digital Video Broadcasting (DVB); Framing structure, channel coding and
modulation for Digital Satellite News Gathering (DSNG) and other contribution applications by
satellite".
[4] ETSI EN 301 192: "Digital Video Broadcasting (DVB); DVB specification for data broadcasting".
[5] ETSI EN 300 429: "Digital Video Broadcasting (DVB); Framing structure, channel coding and
modulation for cable systems".
[6] ETSI EN 301 790: "Digital Video Broadcasting (DVB); Interaction channel for satellite
distribution systems".
[7] ETSI ETS 300 801: "Digital Video Broadcasting (DVB); Interaction channel through Public
Switched Telecommunications Network (PSTN)/ Integrated Services Digital Networks (ISDN)".
[8] ETSI EN 301 195: "Digital Video Broadcasting (DVB); Interaction channel through the Global
System for Mobile communications (GSM)".
[9] ETSI ES 200 800: "Digital Video Broadcasting (DVB); DVB interaction channel for Cable TV
distribution systems (CATV)".
[10] Recommendation ITU-R SNG.770-1: "Uniform operational procedures for satellite news gathering
(SNG)".
ETSI

|     |     |     | 10  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | ----------------------------------- |
[11]  ETSI ETS 300 802: "Digital Video Broadcasting (DVB); Network-independent protocols for DVB
interactive services".
[12]  ETSI EN 300 468: "Digital Video Broadcasting (DVB); Specification for Service Information (SI)
in DVB systems".
[13]  ETSI TS 101 545-1: "Digital Video Broadcasting (DVB);Second Generation DVB Interactive
Satellite System (DVB-RCS2); Part 1: Overview and System Level specification".
[14]  ETSI EN 302 307-2: "Digital Video Broadcasting (DVB); Second generation framing structure,
channel coding and modulation systems for Broadcasting, Interactive Services, News Gathering
and other broadband satellite applications; Part 2: S2-Extensions (S2X)".
| 2.2  Informative references  |     |     |     |     |
| ---------------------------- | --- | --- | --- | --- |
The following referenced documents are not necessary for the application of the present document but they assist the
user with regard to a particular subject area.
[i.1]  ETSI TS 102 005: "Digital Video Broadcasting (DVB); Specification for the use of Video and
Audio Coding in DVB services delivered directly over IP protocols".
| [i.2]  | Void.  |     |     |     |
| ------ | ------ | --- | --- | --- |
[i.3]  ETSI TR 101 154: "Digital Video Broadcasting (DVB); Implementation guidelines for the use of
MPEG-2 Systems, Video and Audio in satellite, cable and terrestrial broadcasting applications".
[i.4]  ETSI ETR 162: "Digital Video Broadcasting (DVB); Allocation of Service Information (SI) codes
for DVB systems".
[i.5]  ETSI TR 102 376: "Digital Video Broadcasting (DVB) User guidelines for the second generation
system for Broadcasting, Interactive Services, News Gathering and other broadband satellite
applications (DVB-S2)".
| 3  Symbols and abbreviations  |     |     |     |     |
| ----------------------------- | --- | --- | --- | --- |
| 3.1  Symbols                  |     |     |     |     |
For the purposes of the present document, the following symbols apply:
α
Roll-off factor
| γ   |     | Ratio between constellation radii for 16APSK and 32APSK  |     |     |
| --- | --- | -------------------------------------------------------- | --- | --- |
| c   |     | codeword                                                 |     |     |
C/N  Carrier-to-noise power ratio (N measured in a bandwidth equal to symbol rate)
| C/N+I   |         | Carrier-to-(Noise+Interference) ratio  |     |     |
| ------- | ------- | -------------------------------------- | --- | --- |
| d ,d    | ,...,d  | ,d                                     |     |     |
| n −k −1 | n −k −2 | 1 0  BCH code redundancy bits          |     |     |
| bch bch | bch bch |                                        |     |     |
d(x)  BCH code remainder of the division between the generator polynomial and
xn −k
|       |     | bch bch m(x)                |     |     |
| ----- | --- | --------------------------- | --- | --- |
| DFL   |     | Data Field Length           |     |     |
| dmin  |     | LDPC code minimum distance  |     |     |
E /N Ratio between the energy per information bit and single sided noise power
b 0
spectral density
E /N Ratio between the energy per transmitted symbol and single sided noise power
s 0
spectral density
| f   |     | Nyquist frequency  |     |     |
| --- | --- | ------------------ | --- | --- |
N
| f   |     | Carrier frequency  |     |     |
| --- | --- | ------------------ | --- | --- |
0
| G     |     | PLS code generator matrix  |     |     |
| ----- | --- | -------------------------- | --- | --- |
| g(x)  |     | code generator polynomial  |     |     |
ETSI

|     |     |     | 11  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | ----------------------------------- |
g (x), g (x), …, g (x)  polynomials to obtain BCH code generator polynomial
| 1 2         | 12  |                              |     |     |
| ----------- | --- | ---------------------------- | --- | --- |
| i           |     | LDPC code information block  |     |     |
| i ,i ,...,i |     | LDPC code information bits   |     |     |
| 0 1 k       | −1  |                              |     |     |
ldpc
| H(f)  |     | RC filters frequency transfer function  |     |     |
| ----- | --- | --------------------------------------- | --- | --- |
| H     |     | LDPC code parity check matrix           |     |     |
(n-k)xn
I, Q  In-phase, Quadrature phase components of the modulated signal
| K   |     | number of bits of BCH uncoded Block  |     |     |
| --- | --- | ------------------------------------ | --- | --- |
bch
| N   |     | number of bits of BCH coded Block  |     |     |
| --- | --- | ---------------------------------- | --- | --- |
bch
| k ldpc   |     | number of bits of LDPC uncoded Block   |     |     |
| -------- | --- | -------------------------------------- | --- | --- |
| n        |     | number of bits of LDPC coded Block     |     |     |
ldpc
| η   |     | PLFRAMING efficiency  |     |     |
| --- | --- | --------------------- | --- | --- |
η
|     |     | code efficiency  |     |     |
| --- | --- | ---------------- | --- | --- |
c
η
|     |     | number of transmitted bits per constellation symbol  |     |     |
| --- | --- | ---------------------------------------------------- | --- | --- |
MOD
η
|     |     | System spectral efficiency  |     |     |
| --- | --- | --------------------------- | --- | --- |
tot
| m          |           | BCH code information word            |     |     |
| ---------- | --------- | ------------------------------------ | --- | --- |
| m(x)       |           | BCH code message polynomial          |     |     |
| m ,m       | ,...,m ,m |                                      |     |     |
| −1         | −2        | BCH code information bits            |     |     |
| k bch      | k bch 1 0 |                                      |     |     |
| M          |           | number of modulated symbols in SLOT  |     |     |
| p ,p ,...p |           |                                      |     |     |
|            |           | LDPC code parity bits                |     |     |
| 0 1        | n −k −1   |                                      |     |     |
ldpc ldpc
| P   |     | number of pilot symbols in a pilot block        |     |     |
| --- | --- | ----------------------------------------------- | --- | --- |
| q   |     | code rate dependant constant for LDPC codes     |     |     |
| θ   |     | deviation angle in hierarchical constellations  |     |     |
| r   |     | In-band ripple (dB)                             |     |     |
m
R Symbol rate corresponding to the bilateral Nyquist bandwidth of the
s
modulated signal
| R   |     | Useful bit rate at the DVB-S2 system input   |     |     |
| --- | --- | -------------------------------------------- | --- | --- |
u
| S   |     | Number of Slots in a XFECFRAME  |     |     |
| --- | --- | ------------------------------- | --- | --- |
| T   |     | Symbol period                   |     |     |
s
| 3.2  Abbreviations  |     |     |     |     |
| ------------------- | --- | --- | --- | --- |
For the purposes of the present document, the following abbreviations apply:
| 16APSK  | 16-ary Amplitude and Phase Shift Keying  |     |     |     |
| ------- | ---------------------------------------- | --- | --- | --- |
| 32APSK  | 32-ary Amplitude and Phase Shift Keying  |     |     |     |
| 8PSK    | 8-ary Phase Shift Keying                 |     |     |     |
| ACM     | Adaptive Coding and Modulation           |     |     |     |
| APSK    | Amplitude Phase Shift Keying             |     |     |     |
| ASI     | Asynchronous Serial Interface            |     |     |     |
| AWGN    | Additive White Gaussian Noise            |     |     |     |
| BB      | BaseBand                                 |     |     |     |
| BC      | Backwards-Compatible                     |     |     |     |
NOTE:  Referred to the system allowing partial stream reception by DVB-S receivers.
BCH  Bose-Chaudhuri-Hocquenghem multiple error correction binary block code
| BER   | Bit Error Ratio                                         |     |     |     |
| ----- | ------------------------------------------------------- | --- | --- | --- |
| BPSK  | Binary Phase Shift Keying                               |     |     |     |
| B     | Bandwidth of the frequency Slot allocated to a service  |     |     |     |
S
| BS   | Broadcast Service                         |     |     |     |
| ---- | ----------------------------------------- | --- | --- | --- |
| BSS  | Broadcast Satellite Service               |     |     |     |
| BW   | BandWidth (at -3 dB) of the transponder   |     |     |     |
| CBR  | Constant Bit Rate                         |     |     |     |
| CCM  | Constant Coding and Modulation            |     |     |     |
| CNI  | Carrier to Noise plus Interference ratio  |     |     |     |
| CRC  | Cyclic Redundancy Check                   |     |     |     |
ETSI

12 ETSI EN 302 307-1 V1.4.1 (2014-11)
D Decimal notation
DEMUX DEMUltipleXer
DF Data Field
DFL Data Field Length
DNP Deleted Null Packets
DSNG Digital Satellite News Gathering
DTH Direct To Home
DTT Digital Terrestrial Television
DVB Digital Video Broadcasting project
DVB-S DVB System for satellite broadcasting
NOTE: As specified in EN 300 421 [2].
DVB-S2 second generation DVB System for satellite broadcasting and unicasting
EBU European Broadcasting Union
EIRP Equivalent Isotropic Radiated Power
EN European Norm
FDM Frequency Division Multiplex
FEC Forward Error Correction
FIFO First In First Out
FSS Fixed Satellite Service
GF Galois Field
GS Generic Stream
HDTV High Definition TeleVision
HEX HEXadecimal notation
HPA High Power Amplifier
IBO Input Back Off
IF Intermediate Frequency
IMUX Input MUltipleXer - filter
IP Internet Protocol
IRD Integrated Receiver Decoder
IS Interactive Services
ISCR Input Stream Clock Reference
ISI Input Stream Identifier
ISSY Input Stream SYnchronizer
ISSYI Input Stream SYnchronizer Indicator
ITU International Telecommunications Union
LDPC Low Density Parity Check (codes)
LNB Low Noise Block
LP Low Priority
LSB Least Significant Bit
LTWTA Linearized Travelling Wave Tube Amplifier
MA Mode Adaptation
MIS Multiple Input Stream
MPE Multi-Protocol Encapsulation
MPEG Moving Pictures Experts Group
MSB Most Significant Bit
NOTE: In DVB-S2 the MSB is always transmitted first.
MUX MUltipleX
NA Not Applicable
NBC Non-Backwards-Compatible
NCR Network Clock Reference
NP Null Packets
NPD Null-Packet Deletion
OBO Output Back Off
OCT OCTal notation
OMUX Output MUltipleXer - filter
PAT Program Association Table
PER (MPEG TS) Packet Error Rate
PID Packet IDentifier
PL Physical Layer
ETSI

13 ETSI EN 302 307-1 V1.4.1 (2014-11)
PLL Phase-Locked Loop
PLS Physical Layer Signalling
PMT Program Map Table
PRBS Pseudo Random Binary Sequence
PS Professional Services
PSK Phase Shift Keying
QEF Quasi-Error-Free
QPSK Quaternary Phase Shift Keying
RCS Return Channel via Satellite
RF Radio Frequency
RO Roll-Off
SA Stream Adaptation
SDTV Standard Definition TeleVision
SI Service Information
SIS Single Input Stream
SMATV Satellite Master Antenna TeleVision
SNG Satellite News Gathering
SOF Start Of Frame
SSA Solid State Amplifier
SSB Single SideBand
TDM Time Division Multiplex
TS Transport Stream
TSDT Transport Stream Descriptor Table
TS/GS Transport Stream/Generic Stream
TSN Time Slice Number (See Annex M)
TV TeleVision
TWT Travelling Wave Tube
TWTA Travelling Wave Tube Amplifier
UP User Packet
UPL User Packet Length
VCM Variable Coding and Modulation
4 Transmission system description
4.1 System definition
The System is defined as the functional block of equipment performing the adaptation of the baseband digital signals,
from the output of a single (or multiple) MPEG transport stream multiplexer(s) (ISO/IEC 13818-1 [1]), or from the
output of a single (or multiple) generic data source(s), to the satellite channel characteristics. The System is designed to
support source coding as defined in ISO/IEC 13818 [1], TR 101 154 [i.3] and TS 102 005 [i.1]. Data services may be
transported in Transport Stream format according to EN 301 192 [4] (e.g. using Multi-protocol Encapsulation), or
Generic Stream format.
If the received signal is above the C/N+I threshold, the Forward Error Correction (FEC) technique adopted in the
System is designed to provide a "Quasi Error Free" (QEF) quality target. The definition of QEF adopted for DVB-S2 is
"less than one uncorrected error-event per transmission hour at the level of a 5 Mbit/s single TV service decoder",
approximately corresponding to a Transport Stream Packet Error Ratio PER< 10-7 before de-multiplexer.
ETSI

14 ETSI EN 302 307-1 V1.4.1 (2014-11)
4.2 System architecture
According to figure 1, the DVB-S2 System shall be composed of a sequence of functional blocks as described below.
Mode adaptation shall be application dependent. It shall provide input stream interfacing, Input Stream
Synchronization (optional), null-packet deletion (for ACM and Transport Stream input format only), CRC-8 coding for
error detection at packet level in the receiver (for packetized input streams only), merging of input streams (for Multiple
Input Stream modes only) and slicing into DATA FIELDs. For Constant Coding and Modulation (CCM) and single
input Transport Stream, Mode Adaptation shall consist of a "transparent" DVB-ASI (or DVB-parallel) to logical-bit
conversion and CRC-8 coding. For Adaptive Coding and Modulation (ACM), Mode Adaptation shall be according to
annex D.
A Base-Band Header shall be appended in front of the Data Field, to notify the receiver of the input stream format and
Mode Adaptation type. To be noted that the MPEG multiplex transport packets may be asynchronously mapped to the
Base-Band Frames.
For applications requiring sophisticated merging policies, in accordance with specific service requirements (e.g. Quality
of Service), Mode Adaptation may optionally be performed by a separate device, respecting all the rules of the DVB-S2
specification. To allow standard interfacing between Mode and Stream Adaptation functions, an optional modulator
interface (Mode Adaptation input interface) is defined, according to clauses I.1 (separate signalling circuit) or I.2
(in-band signalling).
Stream adaptation shall be applied, to provide padding to complete a Base-Band Frame and Base-Band Scrambling.
Forward Error Correction (FEC) Encoding shall be carried out by the concatenation of BCH outer codes and LDPC
(Low Density Parity Check) inner codes (rates 1/4, 1/3, 2/5, 1/2, 3/5, 2/3, 3/4, 4/5, 5/6, 8/9, 9/10). Depending on the
application area, the FEC coded block shall have length n = 64 800 bits or 16 200 bits. When VCM and ACM is
ldpc
used, FEC and modulation mode may be changed in different frames, but remains constant within a frame.
Mapping into QPSK, 8PSK, 16APSK and 32APSK constellations shall be applied, depending on the application area.
Gray mapping of constellations shall be used for QPSK and 8PSK.
Physical layer framing shall be applied, synchronous with the FEC frames, to provide Dummy PLFRAME insertion,
Physical Layer (PL) Signalling, pilot symbols insertion (optional) and Physical Layer Scrambling for energy dispersal.
Dummy PLFRAMEs are transmitted when no useful data is ready to be sent on the channel. The System provides a
regular physical layer framing structure, based on SLOTs of M = 90 modulated symbols, allowing reliable receiver
synchronization on the FEC block structure. A slot is devoted to physical layer signalling, including Start-of-Frame
delimitation and transmission mode definition. This mechanism is suitable also for VCM and ACM demodulator
setting. Carrier recovery in the receivers may be facilitated by the introduction of a regular raster of pilot symbols
(P = 36 pilot symbols every 16 SLOTs of 90 symbols), while a pilot-less transmission mode is also available, offering
an additional 2,4 % useful capacity.
Base-Band Filtering and Quadrature Modulation shall be applied, to shape the signal spectrum (squared-root raised
cosine, roll-off factors 0,35 or 0,25 or 0,20) and to generate the RF signal.
ETSI

|     |     |     |     |     |     |     | 15  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |

MODE ADAPTATION
|     | DATA    |     |     |     |     |     |     |     |             | BB  |     |     |     |
| --- | ------- | --- | --- | --- | --- | --- | --- | --- | ----------- | --- | --- | --- | --- |
|     | Single  |     |     |     |     |     |     |     | Signalling  |     |     |     |     |
I n p u t    I n p u t  S tr e a m   N ul l- p a c k e t  D o t t e d  s u b - s y s t e m s   a r e
|     | I n p u t  |     |     |     |     |     | C R C -8 |   Buffer  |     |     |     |     |     |
| --- | ---------- | --- | --- | --- | --- | --- | -------- | --------- | --- | --- | --- | --- | --- |
S t r ea m   in t e rf a c e  S y n c h ro n i s er D e l et i o n   r  no t   r e le v a n t  f o r
|     |      |     |     |     | (ACM, TS)  |     | E n co d e |     |     |     |                          |     |     |
| --- | ---- | --- | --- | --- | ---------- | --- | ---------- | --- | --- | --- | ------------------------ | --- | --- |
|     | ACM  |     |     |     |            |     |            |     |     |     | single transport stream  |     |     |
COMMAND
|     |     |     |     |     |     |     |     |     | Merger  |     | broadcasting  |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ------- | --- | ------------- | --- | --- |
Slicer
applications
Multiple
|     | I n p u t  |      |             |                 | Nu                        | l l- p a c k e t  |             | Buffer  |              |                    |     |                 |       |
| --- | ---------- | ---- | ----------- | --------------- | ------------------------- | ----------------- | ----------- | ------- | ------------ | ------------------ | --- | --------------- | ----- |
|     | s          | I    | n p u t     | I n p u t  S tr | e a m                     |                   | C R C  -8   |         |              |                    |     |                 |       |
|     | St r e a m | in t | e rf a c e  | S y n c h ro n  | i s er   D                | e l et i o n      |             | r       |              |                    |     |                 |       |
|     |            |      |             |                 | (ACM, TS)                 |                   | E n co d e  |         |              |                    |     |                 |       |
|     |            |      |             |                 |                           |                   |             |         | QPSK,        | PL Signalling &    |     | α=0,35, 0,25,   |       |
|     |            |      |             |                 |                           |                   |             |         |   8 P S K ,  |                    |     |                 | 0,20  |
|     |            |      |             |                 | rates 1/4,1/3,2/5         |                   |             |         |              | Pilot insertion    |     |                 |       |
|     |            |      |             |                 |                           |                   |             |         | 1 6 A P S K  | ,                  |     |                 |       |
|     |            |      |             |                 | 1/2, 3/5, 2/3, 3/4, 4/5,  |                   |             |         | 32APSK       |                    |     |                 |       |
5/6, 8/9, 9/10
|     |     |     |     |     |     |     |     |     |     | I   | PL  |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Bit
|     |               | B         | B    |               |       |                |           |          | m a p p e r   |             | S C R A M |             | B B  F i lt e r    |
| --- | ------------- | --------- | ---- | ------------- | ----- | -------------- | --------- | -------- | ------------- | ----------- | --------- | ----------- | ------------------ |
|     | PADDER        |           |      | B C           | H     | L D P          | C   B     | it       |               | Q           |           |             | a n d              |
|     |               | SC R      | A M  | En c o        | de r  | E n co         | d e r  In | te r -   | i n t o       |             | B L E R   |             |                    |
|     |               |           |      | (nbch,kbch)   |       | (nldpc,kldpc)  |           |          | c o n st e l- |             |           | Q           | u a d r a t u r e  |
|     |               | BLER      |      |               |       |                | leaver    |          |               | Dummy       |           | Modulation  |                    |
|     | Mode          |           |      |               |       |                |           |          | l at io n s   |             |           |             |                    |
| Ada | p ta t i on   |           |      |               |       |                |           |          |               | P L F R     | A M E     |             |                    |
|     | In p u t      |           |      |               |       |                |           |          |               | In s e r    | ti on     |             |                    |
| I n | te r fa c e   | STREAM    |      |               |       |                |           |          |               |             |           |             |                    |
|     |   ADAPTATION  |           |      | FEC ENCODING  |       |                |           | MAPPING  |               | PL FRAMING  |           | MODULATION  |                    |
( o p ti o n a l)
|     |     |     |     |     |     | LP stream for  |     |     |     |     |     |     | to the RF  |
| --- | --- | --- | --- | --- | --- | -------------- | --- | --- | --- | --- | --- | --- | ---------- |
BBHEADER
|     |            |     | BBFRAME  |     |     | BC modes  | FECFRAME  |     |     |     | PLFRAME  |     | satellite  |
| --- | ---------- | --- | -------- | --- | --- | --------- | --------- | --- | --- | --- | -------- | --- | ---------- |
|     | DATAFIELD  |     |          |     |     |           |           |     |     |     |          |     | channel    |

Figure 1: Functional block diagram of the DVB-S2 System
| 4.3  | System configurations  |     |     |     |     |     |     |     |     |     |     |     |     |
| ---- | ---------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Table 1 associates the System configurations to the applications areas. According to table 1, at least "Normative"
subsystems and functionalities shall be implemented in the transmitting and receiving equipment to comply with the
present document Guidelines for mode selection are given in TR 102 376 [i.5].
ETSI

|     |     | 16  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| --- | --- | --- | ----------------------------------- | --- | --- |
Table 1: System configurations and application areas
System configurations  Broadcast  Interactive  DSNG  Professional
|       |                                | services  | services  |     | services  |
| ----- | ------------------------------ | --------- | --------- | --- | --------- |
| QPSK  | 1/4,1/3, 2/5                   | O         | N         | N   | N         |
|       | 1/2, 3/5, 2/3, 3/4, 4/5, 5/6,  |           |           |     |           |
|       |                                | N         | N         | N   | N         |
8/9, 9/10
| 8PSK                          | 3/5, 2/3, 3/4, 5/6, 8/9, 9/10  | N   | N               | N   | N   |
| ----------------------------- | ------------------------------ | --- | --------------- | --- | --- |
| 16APSK                        | 2/3, 3/4, 4/5, 5/6, 8/9, 9/10  | O   | N               | N   | N   |
| 32APSK                        | 3/4, 4/5, 5/6, 8/9, 9/10       | O   | N               | N   | N   |
| CCM                           |                                | N   | N (see note 1)  | N   | N   |
| VCM                           |                                | O   | O               | O   | O   |
| ACM                           |                                | NA  | N (see note 2)  | O   | O   |
| FECFRAME (normal)             | 64 800 (bits)                  | N   | N               | N   | N   |
| FECFRAME (short)              | 16 200 (bits)                  | NA  | N               | O   | N   |
| Single Transport Stream       |                                | N   | N (see note 1)  | N   | N   |
| Multiple Transport Streams    |                                | O   | O (see note 2)  | O   | O   |
| Single Generic Stream         |                                | NA  | O (see note 2)  | NA  | O   |
| Multiple Generic Streams      |                                | NA  | O (see note 2)  | NA  | O   |
| Roll-off 0,35, 0,25 and 0,20  |                                | N   | N               | N   | N   |
Input Stream Synchronizer    NA except   O (see note 3)  O (see note 3)  O (see note 3)
(see note 3)
Null Packet Deletion    NA except   O (see note 3)  O (see note 3)  O (see note 3)
(see note 3)
| Dummy Frame insertion  |     | NA except   | N   | N   | N   |
| ---------------------- | --- | ----------- | --- | --- | --- |
(see note 3)
| Wide-band mode   | (see annex M)  | O   | O   | O   | O   |
| ---------------- | -------------- | --- | --- | --- | --- |
N = normative, O = optional, NA = not applicable.
NOTE 1:  Interactive service receivers shall implement CCM and Single Transport Stream.
NOTE 2:  Interactive Service Receivers shall implement ACM at least in one of the two options: Multiple Transport
Streams or Generic Stream (single/multiple input).
NOTE 3:  Normative for single/multiple TS input stream(s) combined with ACM/VCM or for multiple TS input streams
combined with CCM.

Within the present document, a number of configurations and mechanisms are defined as "Optional". Configurations
and mechanisms explicitly indicated as "optional" within the present document, for a given application area, need not be
implemented in the equipment to comply with the present document. Nevertheless, when an "optional" mode or
mechanism is implemented, it shall comply with the specification as given in the present document.
5  Subsystems specification
The subsystem specification description is organized according to the functional block diagram of figure 1.
5.1  Mode adaptation
This sub-system shall perform Input Interfacing, Input Stream Synchronization (optional), Null-packet deletion (for TS
input streams and ACM only), CRC-8 encoding for error detection (for packetized input streams only), input stream
merging (for multiple input streams only) and input stream slicing in DATA FIELDs. Finally, base-band signalling
shall be inserted, to notify the receiver of the adopted Mode Adaptation format.
According to figure 3, the input sequence(s) is (are):
•  Single or multiple Transport Streams (TS).
•
Single or multiple Generic Streams (packetized or continuous).
The output sequence is a BBHEADER (80 bits) followed by a DATA FIELD.
ETSI

17 ETSI EN 302 307-1 V1.4.1 (2014-11)
5.1.1 Input interface
The System, as defined in the present document, shall be delimited by the interfaces given in table 2.
Table 2: System interfaces
Location Interface Interface type Connection Multiplicity
Transmit station Input MPEG [1, 4] Transport Stream from MPEG multiplexer Single or multiple
(see note 1)
Transmit station Input Generic Stream From data sources Single or multiple
(see note 2)
Transmit station Input ACM command From rate control unit Single
(see note 3)
Transmit station Output 70 MHz/140 MHz IF, L-band IF, to RF devices Single or multiple
RF
(see note 4)
Transmit station Input Mode Adaptation from Mode Adaptation Single
block
NOTE 1: For interoperability reasons, the Asynchronous Serial Interface (ASI) with 188 bytes format, data burst
mode (bytes regularly spread over time) is recommended.
NOTE 2: For data services.
NOTE 3: For ACM only. Allows external setting of the ACM transmission mode.
NOTE 4: IF shall be higher than twice the symbol rate.
The input interface subsystem shall map the input electrical format into internal logical-bit format. The first received bit
will be indicated as the Most Significant Bit (MSB).
A Transport Stream shall be characterized by User Packets (UP) of constant length UPL = 188 × 8 bits (one MPEG
packet), the first byte being a Sync-byte (47 ).
HEX
A Generic Stream shall be characterized by a continuous bit-stream or a stream of constant-length User Packets (UP),
with length UPL bits (maximum UPL value 64 K, UPL = 0 means continuous stream, see clause 5.1.5). A variable
D
length packet stream, or a constant length packet exceeding 64 kbit, shall be treated as a continuous stream.
For Generic packetized streams, if a synch-byte is the first byte of the UP, it shall be left unchanged, otherwise a
sync-byte = 0 shall be inserted before each packet, and UPL shall be increased by eight. UPL information may be
D
derived by static modulator setting.
"ACM Command" signalling input shall allow setting, by an external "transmission mode control unit", of the
transmission parameters to be adopted by the DVB-S2 modulator, for a specific portion of input data. ACM command
shall be according to clause D.1.
Mode Adaptation (optional input) shall be a sequence of Data Fields (according to clause 5.1.5), where each individual
Data Field is preceded by a BBHEADER, according to clause 5.1.6 and to figure 3, and Stream Adaptation Command,
according to clause I.1, to allow setting, by an external 3mode adaptation unit", of the transmission parameters to be
adopted by the DVB-S2 modulator, for each specific MA Packet. Mode Adaptation shall be according to clause I.1
(separate signalling circuit) or I.2 (in-band signalling).
5.1.2 Input stream synchronizer (optional, not relevant for single TS - BS)
Data processing in the DVB-S2 modulator may produce variable transmission delay on the user information. The Input
Stream Synchronizer subsystem (optional) shall provide suitable means to guarantee Constant-Bit-Rate (CBR) and
constant end-to-end transmission delay for packetized input streams (e.g. for Transport Streams). This process shall
follow the specification given in annex D. Examples of receiver implementation are given in TR 102 376 [i.5].
5.1.3 Null-Packet Deletion (ACM and Transport Stream only)
For ACM modes and Transport Stream input data format, MPEG null-packets shall be identified (PID = 8191 ) and
D
removed. This allows to reduce the information rate and increase the error protection in the modulator. The process is
carried-out in a way that the removed null-packets can be re-inserted in the receiver in the exact place where they
originally were. This process shall follow the specification given in annex D.
ETSI

18 ETSI EN 302 307-1 V1.4.1 (2014-11)
5.1.4 CRC-8 encoder (for packetized streams only)
If UPL = 0 (continuous generic stream) this sub-system shall pass forward the input stream without modifications.
D
If UPL ≠ 0 the input stream is a sequence of User Packets of length UPL bits, preceded by a sync-byte (the sync-byte
D
being = 0 when the original stream did not contain a sync-byte).
D
The useful part of the UP (excluding the sync-byte) shall be processed by a systematic 8-bit CRC encoder. The
generator polynomial shall be:
g(X) = (X5+X4+X3+X2+1)(X2+X+1)(X+1) = X8+X7+X6+X4+X2+1
The CRC encoder output shall be computed as:
CRC = remainder [X8 u(X) : g(X)]
Where u(X) is the input sequence (UPL - 8 bits) to be systematically encoded. Figure 2 gives a possible implementation
of the CRC generator by means of a shift register.
The register shall be initialized to all zeros before the first bit of each sequence enters the circuit.
The computed CRC-8 shall replace the sync-byte of the following UP. As described in clause 5.1.6, the sync-byte is
copied into the SYNC field of the BBHEADER for transmission.
UPL
S Y UP S Y UP S Y UP
N C N C N C
Replace next
Sync-byte
Compute
CRC-8
B
Switches: in A for UPL-8 bits; in B for 8 bits
A
CRC-8
B
1 2 3 4 5 6 7 8
A
A
UP (excluding sync-byte)
=EXOR
B
Figure 2: Implementation of the CRC-8 encoder
5.1.5 Merger/Slicer
According to figure 3, the Merger/Slicer input stream(s) is (are) organized as Generic continuous Stream(s) or
Packetized Input Stream(s). The UP length is UPL bits (where UPL = 0 means continuous sequence). The input
stream(s) shall be buffered until the Merger/Slicer may read them.
The Slicer shall read (i.e. slice) from its input (single input stream), or from one of its inputs (multiple input streams) a
DATA FIELD, composed of DFL bits (Data Field Length), where:
K -(10 × 8) ≥ DFL ≥0 (K as per table 5, 80 bits are dedicated to the BBHEADER, see clause 5.1.6).
bch bch
The Merger shall concatenate, in a single output, different data fields read and sliced from one of its inputs. In presence
of a single stream, only the slicing functionality applies.
ETSI

19 ETSI EN 302 307-1 V1.4.1 (2014-11)
A DATA FIELD shall be composed of bits taken from a single input port and shall be transmitted in a homogeneous
transmission mode (FEC code and modulation). The Merger/Slicer prioritization policies are application dependent and
shall follow the strategies described in table 4 (Single Transport Stream Broadcast services) and in table D.2 (for other
application areas).
Depending on the applications, the Merger/Slicer shall either allocate a number of input bits equal to the maximum
DATAFIELD capacity (DFL = K -80), thus breaking UPs in subsequent DATAFIELDs, or shall allocate an integer
bch
number of UPs within the DATAFIELD, making the DFL variable within the above specified boundaries.
When a DATA FIELD is not available at the merger/slicer request on any input port, the Physical Layer Framing
sub-system shall generate and transmit a DUMMY PLFRAME (see clause 5.5.1 and table 12).
After Sync-byte replacing by CRC-8 (see clause 5.1.4), it is necessary to provide the receiver a method to recover UP
synchronization (when the receiver is already synchronized to the DATA FIELD). Therefore the number of bits from
the beginning of the DATA FIELD and the beginning of the first complete UP (first bit of the CRC-8) (see figure 3)
shall be detected by the Merger/Slicer and stored in SYNCD field (i.e. SYNC Distance) of the Base-Band Header
(see clause 5.1.6). For example, SYNCD = 0 means that the first USER PACKET is aligned to the DATA FIELD.
D
Time
Generic Continuous Stream
UPL
Packetised Stream
C R UP C R UP C R UP C R UP C R UP
C C C C C
8 8 8 8 8
SYNCD
80 bits DFL
BBHEADER DATA FIELD
MATYPE UPL DFL SYNC SYNCD CRC-8
(2 bytes) (2 bytes) (2 bytes) (1 byte) (2 bytes) (1 byte)
Figure 3: Stream format at the output of the MODE ADAPTER
5.1.6 Base-Band Header insertion
A fixed length base-band Header (BBHEADER) of 10 bytes shall be inserted in front of the DATA FIELD, describing
its format (the maximum efficiency loss introduced by the BBHEADER is 0,25 % for n = 64 800 and 1 % for
ldpc
n = 16 200 assuming inner code rate 1/2).
ldpc
MATYPE (2 bytes): describes the input stream(s) format, the type of Mode Adaptation and the transmission Roll-off
factor, as explained in table 3.
First byte (MATYPE-1):
• TS/GS field (2 bits): Transport Stream Input or Generic Stream Input (packetized or continuous).
• SIS/MIS field (1 bit): Single Input Stream or Multiple Input Stream.
• CCM/ACM field (1 bit): Constant Coding and Modulation or Adaptive Coding and Modulation (VCM is
signalled as ACM).
• ISSYI (1 bit), (Input Stream Synchronization Indicator): If ISSYI = 1 = active, the ISSY field is inserted after
UPs (see annex D).
ETSI

20 ETSI EN 302 307-1 V1.4.1 (2014-11)
• NPD (1 bit): Null-packet deletion active/not active.
• RO (2 bits): Transmission Roll-off factor (α).
Second byte (MATYPE-2):
• If SIS/MIS = Multiple Input Stream, then second byte = Input Stream Identifier (ISI); else second byte
reserved.
UPL (2 bytes): User Packet Length in bits, in the range 0 to 65 535.
EXAMPLE 1: 0000 = continuous stream.
HEX
EXAMPLE 2: 000A = UP length of 10 bits.
HEX
EXAMPLE 3: UPL = 188x8 for MPEG transport stream packets.
D
DFL (2 bytes): Data Field Length in bits, in the range 0 to 58 112.
EXAMPLE 4: 000A = Data Field length of 10 bits.
HEX
SYNC (1 byte): copy of the User Packet Sync-byte:
• for packetized Transport or Generic Streams: copy of the User Packet Sync byte;
• for Continuous Generic Streams: SYNC= 00 - B8 reserved for transport layer protocol signalling according to
Reference ETR 162 [i.4]; SYNC= B9-FF user private).
EXAMPLE 5: SYNC = 47 for MPEG transport stream packets.
HEX
EXAMPLE 6: SYNC = 00 when the input Generic packetized stream did not contain a sync-byte (therefore
HEX
the receiver, after CRC-8 decoding, shall remove the CRC-8 field without reinserting the
Sync-byte).
SYNCD (2 bytes):
• for packetized Transport or Generic Streams: distance in bits from the beginning of the DATA FIELD and the
first UP from this frame (first bit of the CRC-8). SYNCD = 65535 means that no UP starts in the DATA
D
FIELD;
• for Continuous Generic Streams: SYNCD= 0000 - FFFF reserved for future uses.
CRC-8 (1 byte): error detection code applied to the first 9 bytes of the BBHEADER.
CRC-8 shall be computed using the encoding circuit of figure 2 (switch in A for 72 bits, in B for 8 bits).
The BBHEADER transmission order is from the MSB of the TS/GS field.
Table 4 shows the BBHEADER and the slicing policy for a Single Transport Stream Broadcast Service. For other
application areas, BBHEADERs and merging/slicing policies are defined in table D.2.
Table 3: MATYPE-1 field mapping
TS/GS SIS/MIS CCM/ACM ISSYI NPD RO
11 = Transport 1 = single 1 = CCM 1 = active 1 = active 00 = 0,35
00 = Generic Packetized 0 = multiple 0 = ACM 0 = not-active 0 = not-active 01 = 0,25
01 = Generic continuous 10 = 0,20
10 = reserved 11 = reserved
ETSI

|     |     | 21  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | ----------------------------------- |
Table 4: BBHeader (Mode Adaptation characteristics) and
Slicing Policy for Single Transport Stream Broadcast services
Application  MATYPE-1  MATYPE-2  UPL  DFL  SYNC  SYNCD  CRC-8  Slicing
area/configuration  policy
Broadcasting services /  11-1-1-0-0-Y  XXXXXXXX  188 x8  K  -80   47   Y  Y  Break
|     |     | D bch D | HEX |
| --- | --- | ------- | --- |
CCM, single TS   No timeout
No Padding
No Dummy
frame
X= not defined; Y = according to configuration/computation.
Break = break packets in subsequent DATAFIELDs; Timeout: maximum delay in merger/slicer buffer.

| 5.2  Stream adaptation  |     |     |     |
| ----------------------- | --- | --- | --- |
Stream adaptation (see figures 1 and 4) provides padding to complete a constant length (K  bits) BBFRAME and
bch
scrambling. K  depends on the FEC rate, as reported in table 5. Padding may be applied in circumstances when the
bch
user data available for transmission are not sufficient to completely fill a BBFRAME, or when an integer number of
UPs has to be allocated in a BBFRAME.
The input stream shall be a BBHEADER followed by a DATA FIELD. The output stream shall be a BBFRAME.

|     | 80 bits  | DFL  | K -DFL-80  |
| --- | -------- | ---- | ---------- |
bch
|     |     | DATA FIELD  | PADDING  |
| --- | --- | ----------- | -------- |
BBHEADER
|     | BBFRAME  | (K bch  bits)  |     |
| --- | -------- | -------------- | --- |

Figure 4: BBFRAME format at the output of the STREAM ADAPTER
| 5.2.1  Padding  |     |     |     |
| --------------- | --- | --- | --- |
(K -DFL-80) zero bits shall be appended after the DATA FIELD. The resulting BBFRAME shall have a constant
bch
length of K  bits. For Broadcast Service applications, DFL = K  -80, therefore no padding shall be applied.
bch bch
| 5.2.2  BB scrambling  |     |     |     |
| --------------------- | --- | --- | --- |
The complete BBFRAME shall be randomized. The randomization sequence shall be synchronous with the
| BBFRAME, starting from the MSB and ending after K |     |  bits.  |     |
| ------------------------------------------------- | --- | ------- | --- |
bch
The scrambling sequence shall be generated by the feed-back shift register of figure 5. The polynomial for the Pseudo
Random Binary Sequence (PRBS) generator shall be:
|     |     | 1 + X14 + X15  |     |
| --- | --- | -------------- | --- |
Loading of the sequence (100101010000000) into the PRBS register, as indicated in figure 5, shall be initiated at the
start of every BBFRAME.
ETSI

|     |     |       |     |       |                             | 22       |               |           | ETSI EN 302 307-1 V1.4.1 (2014-11)  |      |      |
| --- | --- | ----- | --- | ----- | --------------------------- | -------- | ------------- | --------- | ----------------------------------- | ---- | ---- |
|     |     |       |     |       | In i t  ia l i z a t  io n  |   s e q  | u e  n c e    |           |                                     |      |      |
|     | 1   | 0  0  | 1   | 0  1  | 0                           | 1  0     | 0             | 0         | 0  0                                | 0    | 0    |
|     | 1   | 2  3  | 4   | 5  6  | 7                           | 8  9     | 1 0           | 1 1  1 2  | 1 3                                 | 1 4  | 1 5  |
0   0   0   0   0   0   1   1   . . . .
EXOR
clear BBFRAME input
Randomised BBFRAME output

Figure 5: Possible implementation of the PRBS encoder
| 5.3  | FEC encoding  |     |     |     |     |     |     |     |     |     |     |
| ---- | ------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
This sub-system shall perform outer coding (BCH), Inner Coding (LDPC) and Bit interleaving. The input stream shall
be composed of BBFRAMEs and the output stream of FECFRAMEs.
Each BBFRAME (K  bits) shall be processed by the FEC coding subsystem, to generate a FECFRAME (n  bits).
|     |     | bch |     |     |     |     |     |     |     |     | ldpc |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- |
The parity check bits (BCHFEC) of the systematic BCH outer code shall be appended after the BBFRAME, and the
parity check bits (LDPCFEC) of the inner LDPC encoder shall be appended after the BCHFEC field, as shown in
figure 6.

|     |     |     |          |     | N = k |      |         |         |     |          |      |
| --- | --- | --- | -------- | --- | ----- | ---- | ------- | ------- | --- | -------- | ---- |
|     |     |     |          |     | bch   | ldpc |         |         |     |          |      |
|     |     |     |          |     |       |      | N       | -K      |     | n        | -k   |
|     |     |     |          | K   |       |      |         |         |     | ldpc     | ldpc |
|     |     |     |          | bch |       |      |         | bch bch |     |          |      |
|     |     |     | BBFRAME  |     |       |      | BCHFEC  |         |     | LDPCFEC  |      |
|     |     |     |          |     |       | (n   |  bits)  |         |     |          |      |
ldpc

Figure 6: Format of data before bit interleaving
(n  = 64 800 bits for normal FECFRAME, n  = 16 200 bits for short FECFRAME)
|     | ldpc |     |     |     |     |     | ldpc |     |     |     |     |
| --- | ---- | --- | --- | --- | --- | --- | ---- | --- | --- | --- | --- |
Table 5a gives the FEC coding parameters for the normal FECFRAME (n ldpc  = 64 800 bits) and table 5b for the short
| FECFRAME (n |     |  = 16 200 bits).  |     |     |     |     |     |     |     |     |     |
| ----------- | --- | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
ldpc
|     |        | Table 5a: Coding parameters (for normal FECFRAME n |     |                   |     |     |     |      | ldpc |  = 64 800)        |     |
| --- | ------ | -------------------------------------------------- | --- | ----------------- | --- | --- | --- | ---- | ---- | ----------------- | --- |
|     | LDPC   | BCH Uncoded                                        |     | BCH coded block N |     |     |     | BCH  |      | LDPC Coded Block  |     |
bch
|     | code  | Block K |     |                      |     |     | t-error correction  |     |     |     | n    |
| --- | ----- | ------- | --- | -------------------- | --- | --- | ------------------- | --- | --- | --- | ---- |
|     |       |         | bch | LDPC Uncoded Block k |     |     |                     |     |     |     | ldpc |
ldpc
|     | 1/4   | 16 008  |     |     | 16 200  |     |     | 12  |     |     | 64 800  |
| --- | ----- | ------- | --- | --- | ------- | --- | --- | --- | --- | --- | ------- |
|     | 1/3   | 21 408  |     |     | 21 600  |     |     | 12  |     |     | 64 800  |
|     | 2/5   | 25 728  |     |     | 25 920  |     |     | 12  |     |     | 64 800  |
|     | 1/2   | 32 208  |     |     | 32 400  |     |     | 12  |     |     | 64 800  |
|     | 3/5   | 38 688  |     |     | 38 880  |     |     | 12  |     |     | 64 800  |
|     | 2/3   | 43 040  |     |     | 43 200  |     |     | 10  |     |     | 64 800  |
|     | 3/4   | 48 408  |     |     | 48 600  |     |     | 12  |     |     | 64 800  |
|     | 4/5   | 51 648  |     |     | 51 840  |     |     | 12  |     |     | 64 800  |
|     | 5/6   | 53 840  |     |     | 54 000  |     |     | 10  |     |     | 64 800  |
|     | 8/9   | 57 472  |     |     | 57 600  |     |     | 8   |     |     | 64 800  |
|     | 9/10  | 58 192  |     |     | 58 320  |     |     | 8   |     |     | 64 800  |

ETSI

|     |                                                   |     | 23  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --- | ------------------------------------------------- | --- | --- | --- | ----------------------------------- | --- |
|     | Table 5b: Coding parameters (for short FECFRAME n |     |     |     |  = 16 200)                          |     |
ldpc
LDPC  BCH Uncoded  BCH coded block N BCH  Effective  LDPC Coded
bch
C o d e   Block K   t - e r ro r   LDP C   R a te   B lock
bch LDPC Uncoded Block k
id en t if ie r  ldpc co r r e c ti o n  k / 1 6  2 0 0   n
|       |         |         |     |     | ldpc   | ldpc    |
| ----- | ------- | ------- | --- | --- | ------ | ------- |
| 1/4   | 3 072   | 3 240   |     | 12  | 1/5    | 16 200  |
| 1/3   | 5 232   | 5 400   |     | 12  | 1/3    | 16 200  |
| 2/5   | 6 312   | 6 480   |     | 12  | 2/5    | 16 200  |
| 1/2   | 7 032   | 7 200   |     | 12  | 4/9    | 16 200  |
| 3/5   | 9 552   | 9 720   |     | 12  | 3/5    | 16 200  |
| 2/3   | 10 632  | 10 800  |     | 12  | 2/3    | 16 200  |
| 3/4   | 11 712  | 11 880  |     | 12  | 11/15  | 16 200  |
| 4/5   | 12 432  | 12 600  |     | 12  | 7/9    | 16 200  |
| 5/6   | 13 152  | 13 320  |     | 12  | 37/45  | 16 200  |
| 8/9   | 14 232  | 14 400  |     | 12  | 8/9    | 16 200  |
| 9/10  | NA      | NA      |     | NA  | NA     | NA      |

| 5.3.1  | Outer encoding (BCH)  |     |     |     |     |     |
| ------ | --------------------- | --- | --- | --- | --- | --- |
A t-error correcting BCH (N , K ) code shall be applied to each BBFRAME (K ) to generate an error protected
bch bch bch
packet. The BCH code parameters for n  = 64 800 are given in table 5a and for n  = 16 200 in table 5b.
|     | ldpc |     |     |     | ldpc |     |
| --- | ---- | --- | --- | --- | ---- | --- |
The generator polynomial of the t error correcting BCH encoder is obtained by multiplying the first t polynomials in
| table 6a for n |  = 64 800 and in table 5b for n                  |  = 16 200.  |     |     |             |     |
| -------------- | ------------------------------------------------ | ----------- | --- | --- | ----------- | --- |
|                | ldpc                                             | ldpc        |     |     |             |     |
|                | Table 6a: BCH polynomials (for normal FECFRAME n |             |     |     |  = 64 800)  |     |
ldpc
g (x)  1+x2+x3+x5+x16
1
g (x)  1+x+x4+x5+x6+x8+x16
2
g (x)  1+x2+x3+x4+x5+x7+x8+x9+x10+x11+x16
3
g (x)  1+x2+x4+x6+x9+x11+x12+x14+x16
4
1+x+x2+x3+x5+x8+x9+x10+x11+x12+x16
g 5 (x)
g (x)  1+x2+x4+x5+x7+x8+x9+x10+x12+x13+x14+x15+x16
6
g (x)  1+x2+x5+x6+x8+x9+x10+x11+x13+x15+x16
7
g (x)  1+x+x2+x5+x6+x8+x9+x12+x13+x14+x16
8
g (x)  1+x5+x7+x9+x10+x11+x16
9
g (x)  1+x+x2+x5+x7+x8+x10+x12+x13+x14+x16
10
g (x)  1+x2+x3+x5+x9+x11+x12+x13+x16
11
g (x)  1+x+x5+x6+x7+x9+x11+x12+x16
12

|     | Table 6b: BCH polynomials (for short FECFRAME n |     |     |     |  = 16 200)  |     |
| --- | ----------------------------------------------- | --- | --- | --- | ----------- | --- |
ldpc
g (x)  1+x+x3+x5+x14
1
g (x)  1+x6+x8+x11+x14
2
g (x)  1+x+x2+x6+x9+x10+x14
3
g (x)  1+x4+x7+x8+x10+x12+x14
4
g (x)  1+x2+x4+x6+x8+x9+x11+x13+x14
5
g (x)  1+x3+x7+x8+x9+x13+x14
6
g 7 (x)  1+x2+x5+x6+x7+x10+x11+x13+x14
g (x)  1+x5+x8+x9+x10+x11+x14
8
g (x)  1+x+x2+x3+x9+x10+x14
9
g (x)  1+x3+x6+x9+x11+x12+x14
10
g (x)  1+x4+x11+x12+x14
11
g (x)  1+x+x2+x3+x5+x6+x7+x8+x10+x13+x14
12

ETSI

|     |     |     |     |     |     |     | 24  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |
BCH encoding of information bits m=(m
|     |     |     |     |     | −1    | ,m −2 | ,...,m | ,m )onto a codeword:  |     |     |     |     |     |
| --- | --- | --- | --- | --- | ----- | ----- | ------ | --------------------- | --- | --- | --- | --- | --- |
|     |     |     |     |     | k bch | k bch | 1      | 0                     |     |     |     |     |     |
  c = (m −1 ,m −2 ,...,m ,m ,d −k −1 ,d −k −2 ,...,d ,d ) is achieved as follows:
|     |     | k   | bch | k bch | 1 0 n | bch bch | n bch | bch | 1 0 |     |     |     |     |
| --- | --- | --- | --- | ----- | ----- | ------- | ----- | --- | --- | --- | --- | --- | --- |
•  Multiply the message polynomial m(x) = m xk −1+m xk −2+...+m x+m  by xn −k .
|     |     |     |     |     |     | k   | −1 bch | k   | −2  | bch | 1 0 | bch bch  |     |
| --- | --- | --- | --- | --- | --- | --- | ------ | --- | --- | --- | --- | -------- | --- |
|     |     |     |     |     |     | bch |        |     | bch |     |     |          |     |
|     |     |     | −k  |     |     |     |        |     |     |     | −k  | −1+...+d |     |
•  Divide xn bch bch m(x) by g(x), the generator polynomial. Let d(x)=d xn bch bch x+d be
|     |     |     |     |     |     |     |     |     |     | n −k | −1  | 1   | 0   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- | --- | --- |
|     |     |     |     |     |     |     |     |     |     | bch  | bch |     |     |
the remainder.
| •   | Set the codeword polynomial c(x)= |     |     |     |     | xn −k bchm(x)+d(x).  |     |     |     |     |     |     |     |
| --- | --------------------------------- | --- | --- | --- | --- | -------------------- | --- | --- | --- | --- | --- | --- | --- |
bch
| 5.3.2  |     | Inner encoding (LDPC)  |     |     |     |     |     |     |     |     |     |     |     |
| ------ | --- | ---------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
, i=(i
LDPC encoder systematically encodes an information block of size k ,i ,...,i ) onto a codeword of size
|     |     |     |     |     |     |     |     |     | ldpc | 0 1 | k −1 |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- | ---- | --- | --- |
ldpc
, c=(i
n ,i ,...,i ,p ,p ,...p ). The transmission of the codeword starts in the given order from i  and
| ldpc       | 0   | 1 k | −1 0  | 1   | n −k −1   |     |     |     |     |     |     |     | 0   |
| ---------- | --- | --- | ----- | --- | --------- | --- | --- | --- | --- | --- | --- | --- | --- |
|            |     |     | ldpc  |     | ldpc ldpc |     |     |     |     |     |     |     |     |
| ends with  | p   | −k  | −1 .  |     |           |     |     |     |     |     |     |     |     |
n ldpc ldpc
| LDPC code parameters (n |     |                                   |      | ,k   | ) are given in tables 5a and 5b.  |     |     |     |     |     |                        |     |     |
| ----------------------- | --- | --------------------------------- | ---- | ---- | --------------------------------- | --- | --- | --- | --- | --- | ---------------------- | --- | --- |
|                         |     |                                   | ldpc | ldpc |                                   |     |     |     |     |     |                        |     |     |
| 5.3.2.1                 |     | Inner coding for normal FECFRAME  |      |      |                                   |     |     |     |     |     |                        |     |     |
|                         |     |                                   |      |      | −k                                |     |     |     |     |     | ) for every block of k |     |     |
The task of the encoder is to determine n  parity bits (p ,p ,...,p −k −1
|                      |     |     |           |                                  | ldpc | ldpc |     | 0   | 1 n | ldpc ldpc |     | ldpc |     |
| -------------------- | --- | --- | --------- | -------------------------------- | ---- | ---- | --- | --- | --- | --------- | --- | ---- | --- |
| information bits, (i |     |     | ,i ,...,i | ). The procedure is as follows:  |      |      |     |     |     |           |     |      |     |
|                      |     | 0   | 1 k       | −1                               |      |      |     |     |     |           |     |      |     |
ldpc
| •   |             |     | =   | = =...= |        | =0.  |     |     |     |     |     |     |     |
| --- | ----------- | --- | --- | ------- | ------ | ---- | --- | --- | --- | --- | --- | --- | --- |
|     | Initialize  | p   | p   | p       | p −k   | −1   |     |     |     |     |     |     |     |
|     |             |     | 0 1 | 2       | n ldpc | ldpc |     |     |     |     |     |     |     |
•  Accumulate the first information bit, i
, at parity bit addresses specified in the first row of tables B.1 through
0
B.11 in annex B. For example, for rate 2/3 (table B.6), (all additions are in GF(2)):
|     |     |       | =   | ⊕i     |     |     |     | =     | ⊕i    |     |     |     |     |
| --- | --- | ----- | --- | ------ | --- | --- | --- | ----- | ----- | --- | --- | --- | --- |
|     |     |       | p p |        |     |     | p   | p     |       |     |     |     |     |
|     |     |       | 0   | 0 0    |     |     |     | 2767  | 2767  | 0   |     |     |     |
|     |     |       | =   | ⊕i     |     |     |     | =     | ⊕i    |     |     |     |     |
|     |     | p     | p   |        |     |     |     | p p   |       |     |     |     |     |
|     |     | 10491 |     | 10491  | 0   |     |     | 240   | 240   | 0   |     |     |     |
|     |     | p     | = p | ⊕i     |     |     | p   | = p   | ⊕i    |     |     |     |     |
|     |     | 16043 |     | 16043  | 0   |     |     | 18673 | 18673 | 0   |     |     |     |
|     |     | p     | = p | ⊕i     |     |     | p   | = p   | ⊕i    |     |     |     |     |
|     |     |       | 506 | 506 0  |     |     |     | 9279  | 9279  | 0   |     |     |     |
|     |     | p     | = p | ⊕i     |     |     | p   | = p   | ⊕i    |     |     |     |     |
|     |     | 12826 |     | 12826  | 0   |     |     | 10579 | 10579 | 0   |     |     |     |
|     |     |       | =   | ⊕i     |     |     |     | =     | ⊕i    |     |     |     |     |
|     |     | p     | p   |        |     |     | p   | p     |       |     |     |     |     |
|     |     | 8065  |     | 8065 0 |     |     |     | 20928 | 20928 | 0   |     |     |     |
|     |     |       | =   | ⊕i     |     |     |     |       |       |     |     |     |     |
|     |     | p     | p   |        |     |     |     |       |       |     |     |     |     |
|     |     | 8226  |     | 8226 0 |     |     |     |       |       |     |     |     |     |
•  For the next 359 information bits,  i ,m=1,2,...,359 accumulate i at parity bit addresses
|     |     |     |     |     |     | m   |     |     |     | m   |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
{x+mmod360×q}mod(n −k ) where xdenotes the address of the parity bit accumulator corresponding
|     |     |     |     |     | ldpc ldpc |     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --------- | --- | --- | --- | --- | --- | --- | --- | --- |
to the first bit i , and qis a code rate dependent constant specified in table 7a. Continuing with the example,
0
q=60for rate 2/3. So for example for information bit i
, the following operations are performed,
1
|     |     |       | p = p | ⊕i    |     |     | p   | = p   | ⊕i    |     |     |     |     |
| --- | --- | ----- | ----- | ----- | --- | --- | --- | ----- | ----- | --- | --- | --- | --- |
|     |     |       | 60    | 60 1  |     |     |     | 2827  | 2827  | 1   |     |     |     |
|     |     | p     | = p   | ⊕i    |     |     |     | p = p | ⊕i    |     |     |     |     |
|     |     | 10551 |       | 10551 | 1   |     |     | 300   | 300   | 1   |     |     |     |
|     |     | p     | = p   | ⊕i    |     |     | p   | = p   | ⊕i    |     |     |     |     |
|     |     | 16103 |       | 16103 | 1   |     |     | 18733 | 18733 | 1   |     |     |     |
ETSI

|     |     |     |     |     |     | 25  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- |
|     |     | =   | ⊕i  |     |     |     |     |     |                                     |
p p
|     | 566   | 566   | 1    |     |     | p     | = p   | ⊕i     |     |
| --- | ----- | ----- | ---- | --- | --- | ----- | ----- | ------ | --- |
|     |       |       |      |     |     | 9339  |       | 9339 1 |     |
|     | p     | = p   | ⊕i   |     |     |       |       |        |     |
|     | 12886 | 12886 | 1    |     |     |       | =     | ⊕i     |     |
|     |       |       |      |     |     | p     | p     |        |     |
|     |       |       |      |     |     | 10639 | 10639 | 1      |     |
|     | p     | = p   | ⊕i   |     |     | p     | = p   | ⊕i     |     |
|     | 8125  | 8125  | 1    |     |     | 20988 | 20988 | 1      |     |
|     | p     | = p   | ⊕i   |     |     |       |       |        |     |
|     | 8286  | 8286  | 1    |     |     |       |       |        |     |
•  For the 361st information bit i , the addresses of the parity bit accumulators are given in the second row of
360
the tables B.1 through B.11. In a similar manner the addresses of the parity bit accumulators for the following
| 359 information bits i |     |     | ,m=361,362,...,719 are obtained using the formula  |     |     |     |     |     |     |
| ---------------------- | --- | --- | -------------------------------------------------- | --- | --- | --- | --- | --- | --- |
m
{x+(mmod360)×q}mod(n −k ) where xdenotes the address of the parity bit accumulator
ldpc ldpc
corresponding to the information bit i , i.e. the entries in the second row of the tables B.1 through B.11.
360
•  In a similar manner, for every group of 360 new information bits, a new row from tables B.1 through B.11 are
used to find the addresses of the parity bit accumulators.
After all of the information bits are exhausted, the final parity bits are obtained as follows:
=1.
•  Sequentially perform the following operations starting with i
|                   |     |               | =   | ⊕p        | i=1,2,...,n                    |     | −k   | −1   |      |
| ----------------- | --- | ------------- | --- | --------- | ------------------------------ | --- | ---- | ---- | ---- |
|                   |     |               | p   | p i−1     | ,                              |     |      |      |      |
|                   |     |               | i   | i         |                                |     | ldpc | ldpc |      |
| •                 |     | p, i=0,1,..,n |     | −k        | −1 is equal to the parity bit  |     |      |      |      |
| Final content of  |     |               |     |           |                                |     |      |      | p .  |
|                   |     | i             |     | ldpc ldpc |                                |     |      |      | i    |
Table 7a: q values for normal frames
|     |     |     |     | Code Rate  |       |     | q    |     |     |
| --- | --- | --- | --- | ---------- | ----- | --- | ---- | --- | --- |
|     |     |     |     |            | 1/4   |     | 135  |     |     |
|     |     |     |     |            | 1/3   |     | 120  |     |     |
|     |     |     |     |            | 2/5   |     | 108  |     |     |
|     |     |     |     |            | 1/2   |     | 90   |     |     |
|     |     |     |     |            | 3/5   |     | 72   |     |     |
|     |     |     |     |            | 2/3   |     | 60   |     |     |
|     |     |     |     |            | 3/4   |     | 45   |     |     |
|     |     |     |     |            | 4/5   |     | 36   |     |     |
|     |     |     |     |            | 5/6   |     | 30   |     |     |
|     |     |     |     |            | 8/9   |     | 20   |     |     |
|     |     |     |     |            | 9/10  |     | 18   |     |     |

| 5.3.2.2  | Inner coding for short FECFRAME  |     |     |     |     |     |     |     |     |
| -------- | -------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
k  BCH encoded bits shall be systematically encoded to generate n bits as described in clause 5.3.2.1, replacing
| ldpc |     |     |     |     |     |     |     | ldpc |     |
| ---- | --- | --- | --- | --- | --- | --- | --- | ---- | --- |
table 7a with table 7b, the tables of annex B with the tables of annex C.
Table 7b: q values for short frames
|     |     |     |     | Code Rate  |      |     | q   |     |     |
| --- | --- | --- | --- | ---------- | ---- | --- | --- | --- | --- |
|     |     |     |     |            | 1/4  |     | 36  |     |     |
|     |     |     |     |            | 1/3  |     | 30  |     |     |
|     |     |     |     |            | 2/5  |     | 27  |     |     |
|     |     |     |     |            | 1/2  |     | 25  |     |     |
|     |     |     |     |            | 3/5  |     | 18  |     |     |
|     |     |     |     |            | 2/3  |     | 15  |     |     |
|     |     |     |     |            | 3/4  |     | 12  |     |     |
|     |     |     |     |            | 4/5  |     | 10  |     |     |
|     |     |     |     |            | 5/6  |     | 8   |     |     |
|     |     |     |     |            | 8/9  |     | 5   |     |     |
ETSI

|        |                                                     |     |     | 26  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| ------ | --------------------------------------------------- | --- | --- | --- | ----------------------------------- | --- |
| 5.3.3  | Bit Interleaver (for 8PSK, 16APSK and 32APSK only)  |     |     |     |                                     |     |
For 8PSK, 16APSK, and 32APSK modulation formats, the output of the LDPC encoder shall be bit interleaved using a
block interleaver. Data is serially written into the interleaver column-wise, and serially read out row-wise (the MSB of
BBHEADER is read out first, except 8PSK rate 3/5 case where MSB of BBHEADER is read out third) as shown in
figures 7 and 8.
The configuration of the block interleaver for each modulation format is specified in table 8.
Table 8: Bit Interleaver structure
Modulation  Rows (for n  = 64 800)  Rows (for n  = 16 200)  Columns
|     |         |     | ldpc    |     | ldpc   |     |
| --- | ------- | --- | ------- | --- | ------ | --- |
|     | 8PSK    |     | 21 600  |     | 5 400  | 3   |
|     | 16APSK  |     | 16 200  |     | 4 050  | 4   |
|     | 32APSK  |     | 12 960  |     | 3 240  | 5   |

MSB
of BBHeader
WRITE
READ
MSB
of BBHeader
read-out
|     | Row 1  |     |     |     |     |     |
| --- | ------ | --- | --- | --- | --- | --- |
first

Row 21600
LSB
|     |     | Column 1  | Column 3  |     |     |     |
| --- | --- | --------- | --------- | --- | --- | --- |
of FECFRAME

Figure 7: Bit Interleaving scheme for 8PSK and normal FECFRAME length (all rates except 3/5)
ETSI

27 ETSI EN 302 307-1 V1.4.1 (2014-11)
MSB
of BBHeader
WRITE
READ
MSB
of BBHeader
read-out third
Row 1
Row 21600
LSB
Column 1 Column 3
of FECFRAME
Figure 8: Bit Interleaving scheme for 8PSK and normal FECFRAME length (rate 3/5 only)
5.4 Bit mapping into constellation
Each FECFRAME (which is a sequence of 64 800 bits for normal FECFRAME, or 16 200 bits for short FECFRAME),
shall be serial-to-parallel converted (parallelism level = η 2 for QPSK, 3 for 8PSK, 4 for 16APSK, 5 for 32APSK)
MOD
in figures 9 to 12, the MSB of the FECFRAME is mapped into the MSB of the first parallel sequence. Each parallel
sequence shall be mapped into constellation, generating a (I,Q) sequence of variable length depending on the selected
modulation efficiency η .
MOD
The input sequence shall be a FECFRAME, the output sequence shall be a XFECFRAME (compleX FECFRAME),
composed of 64 800/η (normal XFECFRAME) or 16 200/η (short XFECFRAME) modulation symbols. Each
MOD MOD
modulation symbol shall be a complex vector in the format (I,Q) (I being the in-phase component and Q the quadrature
component) or in the equivalent format ρ exp(jφ) (ρ being the modulus of the vector and φ being its phase).
5.4.1 Bit mapping into QPSK constellation
For QPSK, the System shall employ conventional Gray-coded QPSK modulation with absolute mapping (no differential
coding). Bit mapping into the QPSK constellation shall follow figure 9. The normalized average energy per symbol
shall be equal to
ρ2
= 1.
Two FECFRAME bits are mapped to a QPSK symbol i.e. bits 2i and 2i+1 determines the ith QPSK symbol, where i = 0,
1, 2, …, (N/2)-1 and N is the coded LDPC block size.
ETSI

28 ETSI EN 302 307-1 V1.4.1 (2014-11)
Q I=MSB Q=LSB
10 00
ρ=1
φ=π/4
I
11 01
Figure 9: Bit mapping into QPSK constellation
5.4.2 Bit mapping into 8PSK constellation
For 8PSK, the System shall employ conventional Gray-coded 8PSK modulation with absolute mapping (no differential
coding). Bit mapping into the 8PSK constellation shall follow figure 10. The normalized average energy per symbol
shall be equal to
ρ2
= 1.
Bits 3i, 3i+1, 3i+2 of the interleaver output determine the ith 8PSK symbol where i = 0, 1, 2,… (N/3)-1 and N is the
coded LDPC block size.
Q
100 MSB
110 LSB
000
ρ=1
010 φ=π/4
001 I
011
101
111
Figure 10: Bit mapping into 8PSK constellation
5.4.3 Bit mapping into 16APSK constellation
The 16APSK modulation constellation (figure 11) shall be composed of two concentric rings of uniformly spaced 4 and
12 PSK points, respectively in the inner ring of radius R and outer ring of radius R .
1 2
The ratio of the outer circle radius to the inner circle radius (γ =R /R ) shall comply with table 9.
2 1
Two are the admitted values for the constellation amplitudes, allowing performance optimization according to the
channel characteristics (e.g. single or multiple carriers per transponder, use of non-linear predistortion):
• E=1 (E=unit average symbol energy) corresponding to [R ]2 + 3[R ]2 = 4.
1 2
• R =1.
2
Bits 4i, 4i+1, 4i+2 and 4i+3 of the interleaver output determine the ith 16APSK symbol, where i = 0, 1, 2, …, (N/4)-1
and N is the coded LDPC block size.
ETSI

|     |     |     | 29  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | ----------------------------------- |
Q
|     |     | 1010 | 1000  |     |
| --- | --- | ---- | ----- | --- |

|     | 0010 |     | 0000 | MSB |
| --- | ---- | --- | ---- | --- |
R
2 LSB
|     | 0110 |       |   1100       | 0100 |
| --- | ---- | ----- | ------------ | ---- |
|     |      | 1110  | R 1          |      |
|     |      |       | φ=π/4 φ=π/12 |      |
  I
text
|     | 0111 | 1111 |     | 0101 |
| --- | ---- | ---- | --- | ---- |
1101
0011
0001
γ

|     | = R 2  / R 1 | 1011 |     |     |
| --- | ------------ | ---- | --- | --- |
1001
Figure 11: 16APSK signal constellation
Table 9: Optimum constellation radius ratio γ (linear channel) for 16APSK
γ
|     | Code rate  | Modulation/coding spectral efficiency  |       |       |
| --- | ---------- | -------------------------------------- | ----- | ----- |
|     | 2/3        |                                        | 2,66  | 3,15  |
|     | 3/4        |                                        | 2,99  | 2,85  |
|     | 4/5        |                                        | 3,19  | 2,75  |
|     | 5/6        |                                        | 3,32  | 2,70  |
|     | 8/9        |                                        | 3,55  | 2,60  |
|     | 9/10       |                                        | 3,59  | 2,57  |

5.4.4  Bit mapping into 32APSK
The 32APSK modulation constellation (see figure 12) shall be composed of three concentric rings of uniformly spaced
4, 12 and 16 PSK points, respectively in the inner ring of radius R , the intermediate ring of radius R  and the outer ring
1 2
| or radius R 3 . Table 10 defines the values of Y. |     | 1  = R 2 / R | 1  and Y. 2  = R 3 / R 1 | .   |
| ------------------------------------------------- | --- | ------------ | ------------------------ | --- |
Two are the admitted values for the constellation amplitudes, allowing performance optimization according to the
channel characteristics (e.g., single or multiple carriers per transponder, use of non-linear predistortion):
| •   |     |     | ]2+ 3[R | ]2+ 4[R ]2 = 8.  |
| --- | --- | --- | ------- | ---------------- |
E=1 (E=unit average symbol energy) corresponding to [R
|     |     |     | 1   | 2 3 |
| --- | --- | --- | --- | --- |
•  R =1.
3
Bits 5i, 5i+1, 5i+2, 5i+3 and 5i+4 of the interleaver output determine the ith 32APSK symbol, where i = 0, 1, 2, (N/5)-1.
ETSI

|     |     |     |     |     |     | 30  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- |
Q
01101
11101
01001
MSB
|     |     |     | 01100  |     |     |        |     | 11001  |     |     |
| --- | --- | --- | ------ | --- | --- | ------ | --- | ------ | --- | --- |
|     |     |     |        | R   |     | 00001  |     |        |     | LSB |
00101
3
|     |     |     | 11100  | 00100  |     |     | 00000  |     | 01000  |     |
| --- | --- | --- | ------ | ------ | --- | --- | ------ | --- | ------ | --- |
R
2
|     |     |        | 10100  |     | 10101  |       | 10001  | 10000  |       |     |
| --- | --- | ------ | ------ | --- | ------ | ----- | ------ | ------ | ----- | --- |
|     |     | 11110  |        |     |        | R     |        |        | 11000 |     |
|     |     |        |        |     |        | 1     |        | φ=π/8  |       |     |
|     |     |        |        |     |        | φ=π/4 | φ=π/12 |        |       |     |
|     |     |        |        |     |        |       |        |        | I     |     |
text
10110
|     |     |     |     |     | 10111  |     | 10011  | 10010  |     |     |
| --- | --- | --- | --- | --- | ------ | --- | ------ | ------ | --- | --- |
01110
|     |     |     |        | 00110  |        |        |     | 00010  | 11010  |     |
| --- | --- | --- | ------ | ------ | ------ | ------ | --- | ------ | ------ | --- |
|     |     |     | 11111  |        | 00111  | 00011  |     | 01010  |        |     |

|     |     | γ       |                 |        |     |     |        |     |     |     |
| --- | --- | ------- | --------------- | ------ | --- | --- | ------ | --- | --- | --- |
|     |     | 1 =     | R 2   /   R 1   |        |     |     |        |     |     |     |
|     |     | γ 2   = | R   /   R       | 01111  |     |     | 11011  |     |     |     |
3 1
|     |     |     |     |     |     | 01011  |     |     |     |     |
| --- | --- | --- | --- | --- | --- | ------ | --- | --- | --- | --- |

Figure 12: 32APSK signal constellation
Table 10: optimum constellation radius ratios γ  and γ  (linear channel) for 32 APSK
|     |            |     |                                        |     |       |     | 1   | 2     |     |       |
| --- | ---------- | --- | -------------------------------------- | --- | ----- | --- | --- | ----- | --- | ----- |
|     | Code rate  |     | Modulation/coding spectral efficiency  |     |       |     |     | γ     |     | γ     |
|     |            |     |                                        |     |       |     |     | 1     |     | 2     |
|     | 3/4        |     |                                        |     | 3,74  |     |     | 2,84  |     | 5,27  |
|     | 4/5        |     |                                        |     | 3,99  |     |     | 2,72  |     | 4,87  |
|     | 5/6        |     |                                        |     | 4,15  |     |     | 2,64  |     | 4,64  |
|     | 8/9        |     |                                        |     | 4,43  |     |     | 2,54  |     | 4,33  |
|     | 9/10       |     |                                        |     | 4,49  |     |     | 2,53  |     | 4,30  |

| 5.5  | Physical Layer (PL) framing  |     |     |     |     |     |     |     |     |     |
| ---- | ---------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
The PLFraming sub-system shall generate a physical layer frame (named PLFRAME) by performing the following
processes (see figures 1 and 13):
•
Dummy PLFRAME generation when no XFECFRAME is ready to be processed and transmitted.
•
XFECFRAME slicing into an integer number S of constant length SLOTs (length: M = 90 symbols each);
S shall be according to table 11.
•  PLHEADER generation and insertion before the XFECFRAME for receiver configuration. PLHEADER shall
occupy exactly one SLOT (length: M = 90 Symbols).
•  Pilot Block insertion (for modes requiring pilots) every 16 SLOTS, to help receiver synchronization. The Pilot
Block shall be composed of P = 36 pilot symbols.
•
Randomization of the (I, Q) modulated symbols by means of a physical layer scrambler.
The input stream of the sub-system shall be a XFECFRAME and the output a scrambled PLFRAME.
ETSI

|     |     |     |     |     | 31  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- |

XFECFRAME
S slots
90 symbols
|     |     | Slot-1  |     | Slot-2  |     | Slot-S  |     |     |
| --- | --- | ------- | --- | ------- | --- | ------- | --- | --- |
1 slot   (π/2BPSK)
|     |           |     |         | 16 slots   (selected modulation)  |         |          | 36 symbols  |         |
| --- | --------- | --- | ------- | --------------------------------- | ------- | -------- | ----------- | ------- |
|     | PLHEADER  |     | Slot-1  |                                   | Slot-…  | Slot-16  | Pilot       | Slot-S  |
block
For modes
unmodulated
| SOF   | PLSCODE  |     |     |     |     |                   |           |     |
| ----- | -------- | --- | --- | --- | --- | ----------------- | --------- | --- |
|       |          |     |     |     |     | requiring pilots  | carriers  |     |
PLFRAME before PL Scrambling          90(S+1)+P int{(S-1)/16}    (P=36 pilots)

Figure 13: Format of a "Physical Layer Frame" PLFRAME
Table 11: S = number of SLOTs (M = 90 symbols) per XFECFRAME
|     |     |              |     | n               |  = 64 800     | n              |  = 16 200     |     |
| --- | --- | ------------ | --- | --------------- | ------------- | -------------- | ------------- | --- |
|     |     |              |     | ldpc            |               | ldpc           |               |     |
|     |     |              |     | (normal frame)  |               | (short frame)  |               |     |
|     | η   |  (bit/s/Hz)  |     | S               | η % no-pilot  | S              | η % no-pilot  |     |
MOD
|     |     | 2   |     | 360   | 99,72  | 90   | 98,90  |     |
| --- | --- | --- | --- | ----- | ------ | ---- | ------ | --- |
|     |     | 3   |     | 240   | 99,59  | 60   | 98,36  |     |
|     |     | 4   |     | 180   | 99,45  | 45   | 97,83  |     |
|     |     | 5   |     | 144   | 99,31  | 36   | 97,30  |     |

The PLFRAMING efficiency is η = 90S/[90(S+1)+ P int{(S-1)/16}], where P = 36 and int{.} is the integer function.
| 5.5.1  | Dummy PLFRAME insertion  |     |     |     |     |     |     |     |
| ------ | ------------------------ | --- | --- | --- | --- | --- | --- | --- |
A Dummy PLFRAME shall be composed of a PLHEADER (see clause 5.5.2) and of 36 SLOTS of un-modulated
carriers (I = (1/√2), Q = (1/√2)).
| 5.5.2  | PL signalling  |     |     |     |     |     |     |     |
| ------ | -------------- | --- | --- | --- | --- | --- | --- | --- |
The PLHEADER is intended for receiver synchronization and physical layer signalling.
NOTE:  After decoding the PLHEADER, the receiver knows the PLFRAME duration and structure, the
modulation and coding scheme of the XFECFRAME, the presence or absence of pilot symbols.
The PLHEADER (one SLOT of 90 symbols) shall be composed of the following fields:
•
SOF (26 symbols), identifying the Start of Frame.
•  PLS code (64 symbol): PLS (Physical Layer Signalling) code shall be a non-systematic binary code of length
64 and dimension 7 with minimum distance d  = 32. It is equivalent to the first order Reed-Muller under
min
permutation. It transmits 7 bits for physical layer signalling purpose. These 7 bits consists of two fields:
MODCOD and TYPE defined as follows:
-  MODCOD (5 symbols), identifying the XFECFRAME modulation and FEC rate;
-  TYPE (2 symbols), identifying the FECFRAME length (64 800 bits or 16 200 bits) and the
presence/absence of pilots.
ETSI

|     |     |     |     |     |     | 32  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |
) shall be modulated into 90 π/2BPSK symbols
| The PLHEADER, represented by the binary sequence (y |     |     |     |     |     | 1 , y 2 ,...y | 90  |     |     |     |     |     |
| --------------------------------------------------- | --- | --- | --- | --- | --- | ------------- | --- | --- | --- | --- | --- | --- |
according to the rule:
|           |            |  = (1/√2) (1-2y |     |      |               |  = - (1/√2) (1-2y |     |                          |     |     |     |     |
| --------- | ---------- | --------------- | --- | ---- | ------------- | ----------------- | --- | ------------------------ | --- | --- | --- | --- |
|           | I.         |  = Q.           |     |      | ), I.  = - Q. |                   |     | ) for i = 1, 2, ..., 45  |     |     |     |     |
|           | 2i-1       | 2i-1            |     | 2i-1 | 2i            | 2i                |     | 2i                       |     |     |     |     |
|  5.5.2.1  | SOF field  |                 |     |      |               |                   |     |                          |     |     |     |     |
SOF shall correspond to the sequence 18D2E82  (01-1000-....-0010 in binary notation, the left-side bit being the
HEX
MSB of the PLHEADER).
| 5.5.2.2  | MODCOD field  |     |     |     |     |     |     |     |     |     |     |     |
| -------- | ------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
MODCOD shall correspond to 5 bits, identifying code rates in the set η
= [1/4, 1/3, 2/5, 1/2, 3/5, 2/3, 3/4, 4/5, 5/6,
C
8/9, 9/10] and modulations in the set of spectrum efficiencies η
 = [2, 3, 4, 5] according to table 12.
MOD
Table 12: MODCOD coding
|           | Mode  | MOD  |             | Mode  |     | MOD            | Mode  |     | MOD  |               | Mode  | MOD  |
| --------- | ----- | ---- | ----------- | ----- | --- | -------------- | ----- | --- | ---- | ------------- | ----- | ---- |
|           |       | COD  |             |       |     | COD            |       |     | COD  |               |       | COD  |
| QPSK 1/4  |       | 1    |   QPSK 5/6  |       |     | 9   8PSK 9/10  |       |     | 17   |   32APSK 4/5  |       | 25   |
|           |       | D    |             |       |     | D              |       |     |      | D             |       | D    |
QPSK 1/3  2   QPSK 8/9  10   16APSK 2/3  18   32APSK 5/6  26
|     |     | D   |     |     |     | D   |     |     |     | D   |     | D   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
QPSK 2/5  3   QPSK 9/10  11   16APSK 3/4  19   32APSK 8/9  27
|     |     | D   |     |     |     | D   |     |     |     | D   |     | D   |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
QPSK 1/2  4   8PSK 3/5  12   16APSK 4/5  20   32APSK 9/10  28
|           |     | D   |             |     |     | D                |     |     |     | D           |     | D    |
| --------- | --- | --- | ----------- | --- | --- | ---------------- | --- | --- | --- | ----------- | --- | ---- |
| QPSK 3/5  |     | 5   |   8PSK 2/3  |     |     | 13   16APSK 5/6  |     |     | 21  |   Reserved  |     | 29   |
|           |     | D   |             |     |     | D                |     |     |     | D           |     | D    |
| QPSK 2/3  |     | 6   |   8PSK 3/4  |     |     | 14   16APSK 8/9  |     |     | 22  |   Reserved  |     | 30   |
|           |     | D   |             |     |     | D                |     |     |     | D           |     | D    |
QPSK 3/4  7 D   8PSK 5/6  15 D   16APSK 9/10  23 D   Reserved   31 D
| QPSK 4/5  |     | 8   |   8PSK 8/9  |     |     | 16   32APSK 3/4  |     |     | 24  |   DUMMY  |     | 0   |
| --------- | --- | --- | ----------- | --- | --- | ---------------- | --- | --- | --- | -------- | --- | --- |
|           |     | D   |             |     |     | D                |     |     |     | D        |     | D   |
PLFRAME

| 5.5.2.3  | TYPE field  |     |     |     |     |     |     |     |     |     |     |     |
| -------- | ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
The MSB of the TYPE field shall identify 2 FECFRAME sizes (0 = normal: 64 800 bits; 1 = short: 16 200 bits). The
LSB of the TYPE field shall identify the pilot configurations (see clause 5.5.3) (0 = no pilots, 1 = pilots).
| 5.5.2.4  | PLS code  |     |     |     |     |     |     |     |     |     |     |     |
| -------- | --------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
The MODCODE and TYPE fields are bi-orthogonally coded with a (64,7) code. Such code is constructed starting from
a bi-orthogonal (32,6) code according to the construction in figure 13a.

|     | b   |     | (y , y | , y ,...., y |     | )   |     |     |     |     |     |     |
| --- | --- | --- | ------ | ------------ | --- | --- | --- | --- | --- | --- | --- | --- |
|     | 1   |     | 1      | 2 3          | 32  |     |     |     |     |     |     |     |
(32,6)
|     |     |       |     |     |     |     |     | (y  | , y 1⊗b | ,y , y | 2⊗b ,., y 32⊗b | )   |
| --- | --- | ----- | --- | --- | --- | --- | --- | --- | ------- | ------ | -------------- | --- |
|     |     | code  |     |     |     |     |     |     | 1       | 7 2    | 7              | 7   |
EXOR
b 6
|     |     |     |     |     |     | 1-bit  |     | Parallel  |     |     |     |     |
| --- | --- | --- | --- | --- | --- | ------ | --- | --------- | --- | --- | --- | --- |
|     | b   |     |     |     |     |        |     | to        |     |     |     |     |
|     | 7   |     |     |     |     | Delay  |     |           |     |     |     |     |
serial

Figure 13a: (the symbol ⊗ stands for binary EXOR)
The particular construction guarantees that each odd bit in the (64,7) code is either always equal to the previous one or
is always the opposite. Which of the two hypotheses is true depends on the bit b . This fact can be exploited in case
7
differentially coherent detection is adopted in the receiver.
The MODCOD and the MSB of the TYPE field shall be encoded by a linear block code of length 32 with the following
generator matrix.
ETSI

33 ETSI EN 302 307-1 V1.4.1 (2014-11)
⎡01010101010101010101010101010101⎤
⎢ ⎥
00110011001100110011001100110011
⎢ ⎥
⎢ ⎥
00001111000011110000111100001111
G = ⎢ ⎥
⎢00000000111111110000000011111111⎥
⎢ ⎥
00000000000000001111111111111111
⎢ ⎥
⎢⎣11111111111111111111111111111111⎥⎦
Figure 13b
The most significant bit of the MODCOD is multiplied with the first row of the matrix, the following bit with the
( L )
second row and so on. The 32 coded bits is denoted as y y y . When the least significant bit of the TYPE field is
1 2 32
( L )
0, the final PLS code will generate y y y y y y as the output, i.e. each symbol shall be repeated. When the
1 1 2 2 32 32
( L )
least significant bit of the TYPE field is 1, the final PLS code will generate y y y y y y as output, i.e. the
1 1 2 2 32 32
repeated symbol is further binary complemented. The 64 bits output of the PLS code is further scrambled by the binary
sequence:
0111000110011101100000111100100101010011010000100010110111111010.
5.5.3 Pilots insertion
Two PLFRAME configurations shall be possible:
• Without pilots.
• With pilots.
In this latter case a PILOT BLOCK shall be composed of P = 36 pilot symbols. Each pilot shall be an un-modulated
symbol, identified by I = (1/√2), Q = (1/√2). The first PILOT BLOCK shall be inserted 16 SLOTs after the
PLHEADER, the second after 32 SLOTs and so on, as represented in figure 13. If the PILOT BLOCK position
coincides with the beginning of the next SOF, then the PILOT BLOCK is not transmitted.
The pilot presence/absence in VCM and ACM can be changed on a frame-by-frame basis.
5.5.4 Physical layer scrambling
Prior to modulation, each PLFRAME, excluding the PLHEADER, shall be randomized for energy dispersal by
multiplying the (I+jQ) samples by a complex randomization sequence (C+jC ):
I Q
I = [I C - Q C ]; Q = (I C + Q C)
SCRAMBLED I Q SCRAMBLED Q I
NOTE 1: The randomization sequence rate corresponds to the I-Q PLFRAME symbol rate, thus it has no impact on
the occupied signal bandwidth. The randomization sequence has a period greater than the maximum
required duration of about 70 000 symbols).
The randomization sequence shall be reinitialized at the end of each PLHEADER (see figure 14). The PLFRAME
duration depends on the modulation selected, thus the randomization sequence length shall be truncated to the current
PLFRAME length.
ETSI

|     |     |     |     | 34  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | --- | --- | --- | ----------------------------------- |

| 1 slot  |     |     |     |     |     |     | P =36  |
| ------- | --- | --- | --- | --- | --- | --- | ------ |
Slot-S
| PLHEADER  | Slot-1  |     | Slot-….  |     | Slot-16  |     | Pilot  |
| --------- | ------- | --- | -------- | --- | -------- | --- | ------ |
block
90 symbols
SCRAMBLING SEQUENCE ACTIVE
Scrambling
(scrambled) PLFRAME
RESET

Figure 14: PL SCRAMBLING
The scrambling code sequences shall be constructed by combining two real m-sequences (generated by means of two
generator polynomials of degree 18) into a complex sequence. The resulting sequences thus constitute segments of a set
of Gold sequences.
Let x and y be the two sequences respectively. The x sequence is constructed using the primitive (over GF(2))
polynomial 1+x7+x18. The y sequence is constructed using the polynomial 1+ y5+ y7+ y10+ y18.
The sequence depending on the chosen scrambling code number n is denoted z n  in the sequel. Furthermore, let x(i), y(i)
(i) denote the th symbol of the sequence x, y, and z
and z n i n  respectively. The m-sequences x and y are constructed as:
•  Initial conditions:
-  x is constructed with x(0) = 1, x(1) = x(2) = ... = x(16) = x(17) = 0.
-  y(0) = y(1) = … = y(16) = y(17) = 1.
•
Recursive definition of subsequent symbols:
-  x(i+18) = x(i+7) + x(i) modulo 2, i = 0, …, 218 - 20.
-  y(i+18) = y(i+10) + y(i+7) + y(i+5) + y(i) modulo 2, i = 0, …, 218 - 20.
| The nth Gold code sequence z |  n = 0,1,2,…,218-2, is then defined as:  |     |     |     |     |     |     |
| ---------------------------- | ---------------------------------------- | --- | --- | --- | --- | --- | --- |
n
-  z  (i) = [x((i+n) modulo (218-1)) + y(i)] modulo 2, i = 0,…, 218 - 2.
n
These binary sequences are converted to integer valued sequences R  (R  assuming values 0, 1, 2, 3) by the following
|     |     |     |     |     | n n |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
transformation:
  R (i) = 2 z ((i + 131 072) modulo (218-1)) + z (i)   i = 0, 1, …, 66 419.
| n n                                                         |     |     |     | n                    |     |     |     |
| ----------------------------------------------------------- | --- | --- | --- | -------------------- | --- | --- | --- |
| Finally, the nth complex scrambling code sequence C(i) + jC |     |     |     | (i) is defined as:   |     |     |     |
|                                                             |     |     | I   | Q                    |     |     |     |
 (i) π/2)
|     |     | C(i) + jC I | Q (i) = exp(j R | n         |     |           |     |
| --- | --- | ----------- | --------------- | --------- | --- | --------- | --- |
|     | R   |   exp(j R   |  π/2)           | I         |     | Q         |     |
|     |     | n           | n               | scrambled |     | scrambled |     |
|     | 0   |             | 1               | I         |     | Q         |     |
|     | 1   |             | j               | -Q        |     | I         |     |
|     | 2   |             | -1              | -I        |     | -Q        |     |
|     | 3   |             | -j              | Q         |     | -I        |     |

Figure 15 gives a possible block diagram for PL scrambling sequences generation for n = 0.
ETSI

|     |     |     |     |     | 35  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- |

X(17)
X(0)
z(i)  n
|     | D D | D D D | D D D | D D | D   | D D D | D D D D |     |     |
| --- | --- | ----- | ----- | --- | --- | ----- | ------- | --- | --- |
1+X7+X18
2-bit
adder
1+Y5+Y7+Y10+Y18
R(i)  n
|     | D D    | D D D | D D D | D D | D   | D D D | D D D D |     |       |
| --- | ------ | ----- | ----- | --- | --- | ----- | ------- | --- | ----- |
|     |        |       |       |     |     |       |         |     | x  2  |
|     | Y(17)  |       |       |     |     |       | Y(0)    |     |       |
z(i+131072 mod(218-1))
n
Initialisation
X(0)=1, X(1)=X(2)=...=X(17)=0
Y(0)=Y(1)=...=Y(17)=1

Figure 15: Configuration of PL scrambling code generator for n = 0
In case of broadcasting services, n = 0 shall be used as default sequence, to avoid manual receiver setting or
synchronization delays.
NOTE 2:  n, assuming values in the range 0 to 262 141, indicates the spreading sequence number. The use of
different PL Scrambling sequences allows a reduction of interference correlation between different
services. For the same purpose, it is possible to reuse a shifted version of the same sequence in different
satellite beams. Furthermore n can be unequivocally associated to each satellite operator or satellite or
transponder, thus permitting identification of an interfering signal via the PL Scrambling "signature"
detection. There is no explicit signalling method to convey n to the receiver.
| 5.6  | Baseband shaping and quadrature modulation  |     |     |     |     |     |     |     |     |
| ---- | ------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
After randomization, the signals shall be square root raised cosine filtered. The roll-off factor shall be α = 0,35, 0,25
and 0,20, depending on the service requirements.
The baseband square root raised cosine filter shall have a theoretical function defined by the following expression:
( )
|     |     |     |     | H(f)=1 |     |     |     | for  f < | f 1−α  |
| --- | --- | --- | --- | ------ | --- | --- | --- | -------- | ------ |
N
|     |     |     |             | ⎧     |     | ⎤⎫        |     |      |         |
| --- | --- | --- | ----------- | ----- | --- | --------- | --- | ---- | ------- |
|     |     |     |             | ⎪     |     | ⎡ ⎪1      |     |      |         |
|     |     |     |             |       |     | ⎢ − ⎥     | 2   |      |         |
|     |     |     |             | ⎨ 1 1 | π   | f f ⎬     |     |      |         |
|     |     |     | H(f)=⎪⎩     | +     |     | ⎢ N ⎥ ⎦⎪⎭ |     |      |         |
|     |     |     |             |       | sin | ⎣         |     |      | ( 1−α ) |
|     |     |     |             | 2 2   | 2 f | α         |     | for  | f       |
|     |     |     |             |       | N   |           |     |      | N       |
|     |     |     |             |       |     | ( )       |     |      |         |
|     |     |     | H(f)=0 for  |       | f > | f 1+α,    |     |      |         |
N
|         | 1   | R                                                             |     |     |     |     |     |     |     |
| ------- | --- | ------------------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- |
| where:  | f = | = s  is the Nyquist frequency and  α is the roll-off factor.  |     |     |     |     |     |     |     |
N
|     | 2T  | 2   |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
s
A template for the signal spectrum at the modulator output is given in annex A.
Quadrature modulation shall be performed by multiplying the in-phase and quadrature samples (after baseband
filtering) by sin (2πf t) and cos (2πf t), respectively (where f  is the carrier frequency). The two resulting signals shall
|     |     | 0   | 0   |     |     | 0   |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
be added to obtain the modulator output signal.
ETSI

|                       |     |     | 36  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --------------------- | --- | --- | --- | ----------------------------------- |
| 6  Error performance  |     |     |     |                                     |
Table 13 summarizes performance requirements at QEF over AWGN (E  = average energy per transmitted symbol).
s
Ideal E /No (dB) is the figure achieved by computer simulation, 50 LDPC fixed point decoding iterations
s
(see TR 102 376 [i.5]), perfect carrier and synchronization recovery, no phase noise, AWGN channel. For short
FECFRAMEs an additional degradation of 0,2 dB to 0,3 dB has to be taken into account.
For calculating link budgets, specific satellite channel impairments should be taken into account.
PER is the ratio between the useful transport stream packets (188 bytes) correctly received and affected by errors, after
forward error correction.
Table 13: E /No performance at Quasi Error Free PER = 10-7 (AWGN channel)
s
|     | Mode  | Spectral efficiency  |     | Ideal E s/No (dB)  |
| --- | ----- | -------------------- | --- | ------------------ |
for FECFRAME length = 64 800
|     | QPSK 1/4      | 0,490243  |     | -2,35   |
| --- | ------------- | --------- | --- | ------- |
|     | QPSK 1/3      | 0,656448  |     | -1,24   |
|     | QPSK 2/5      | 0,789412  |     | -0,30   |
|     | QPSK 1/2      | 0,988858  |     | 1,00    |
|     | QPSK 3/5      | 1,188304  |     | 2,23    |
|     | QPSK 2/3      | 1,322253  |     | 3,10    |
|     | QPSK 3/4      | 1,487473  |     | 4,03    |
|     | QPSK 4/5      | 1,587196  |     | 4,68    |
|     | QPSK 5/6      | 1,654663  |     | 5,18    |
|     | QPSK 8/9      | 1,766451  |     | 6,20    |
|     | QPSK 9/10     | 1,788612  |     | 6,42    |
|     | 8PSK 3/5      | 1,779991  |     | 5,50    |
|     | 8PSK 2/3      | 1,980636  |     | 6,62    |
|     | 8PSK 3/4      | 2,228124  |     | 7,91    |
|     | 8PSK 5/6      | 2,478562  |     | 9,35    |
|     | 8PSK 8/9      | 2,646012  |     | 10,69   |
|     | 8PSK 9/10     | 2,679207  |     | 10,98   |
|     | 16APSK 2/3    | 2,637201  |     | 8,97    |
|     | 16APSK 3/4    | 2,966728  |     | 10,21   |
|     | 16APSK 4/5    | 3,165623  |     | 11,03   |
|     | 16APSK 5/6    | 3,300184  |     | 11,61   |
|     | 16APSK 8/9    | 3,523143  |     | 12,89   |
|     | 16APSK 9/10   | 3,567342  |     | 13,13   |
|     | 32APSK 3/4    | 3,703295  |     | 12,73   |
|     | 32APSK 4/5    | 3,951571  |     | 13,64   |
|     | 32APSK 5/6    | 4,119540  |     | 14,28   |
|     | 32APSK 8/9    | 4,397854  |     | 15,69   |
|     | 32APSK 9/10   | 4,453027  |     | 16,05   |
Given the system spectral efficiency η
|     | NOTE:  |     |     |  the ratio between the energy  |
| --- | ------ | --- | --- | ------------------------------ |
tot
per information bit and single sided noise power spectral density
(η
|     | E b /N 0 |  = E s /N 0  - 10log | 10 tot ).  |     |
| --- | -------- | -------------------- | ---------- | --- |

Spectral efficiencies (per unit symbol rate) are computed for normal FECFRAME length and no pilots.
Examples of possible use of the System are given in TR 102 376 [i.5].
ETSI

|     |     |     |     | 37  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| --- | --- | --- | --- | --- | ----------------------------------- | --- | --- |
Annex A (normative):
Signal spectrum at the modulator output
For roll-off factor α = 0,35, the signal spectrum at the modulator output shall be in accordance with EN 300 421 [2].
As an option, the signal spectrum can correspond to a narrower roll-off factor α = 0,25 or α = 0,20.
Figure A.1 gives a template for the signal spectrum at the modulator output.
Figure A.1 also represents a possible mask for a hardware implementation of the Nyquist modulator filter. The points
A to S shown on figures A.1 and A.2 are defined in table A.1. The mask for the filter frequency response is based on the
assumption of ideal Dirac delta input signals, spaced by the symbol period T  = 1/R  = 1/2f  while in the case of
|     |     |     |     |     | S S | N   |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
rectangular input signals a suitable x/sin x correction shall be applied on the filter response.
Relative power (dB)
10
A C
|     |     | E G | I   |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
J
0
|     | B D | F   |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
H L
K
-10
|     |     |     | M   | P   |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
|     | -20 |     |     | Q   |     |     |     |
-30
N
-40
S
-50
|     | 0   | 0,5 | 1   | 1,5 | 2   | 2,5 | 3   |
| --- | --- | --- | --- | --- | --- | --- | --- |
f/f
|     |     |     |     | N   |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
Figure A.1: Template for the signal spectrum mask at the modulator output represented in the
baseband frequency domain, the frequency axis is calibrated for roll-off factor α = 0,35
Figure A.2 gives a mask for the group delay for the hardware implementation of the Nyquist modulator filter.
ETSI

|     |                      |     |     |     |     | 38  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | -------------------- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |
|     | Group delay  x  f    |     | N   |     |     |     |     |     |                                     |     |     |     |
0,2
L
0,15
0,1
J
|     | 0,05  A  | C   | E   | G   | I   |     |     |     |     |     |     |     |
| --- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
0
|     | 0,00  |     | 0,50  |     | 1,00  |     | 1,50  | 2,00  |     | 2,50  |     | 3,00  |
| --- | ----- | --- | ----- | --- | ----- | --- | ----- | ----- | --- | ----- | --- | ----- |
-0,05
|     | B   | D   | F   | H   |     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
K
-0,1
-0,15
M
-0,2
f / f
|     |     |     |     |     |     |     | N   |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Figure A.2: Template of the modulator filter group delay
Table A.1: Definition of points given in figures A.1 and A.2
Point  Frequency   Frequency  Frequency  Relative power  Group delay
|     |     | for α = 0,35  |     |     | for α = 0,25  |     | for α = 0,20  |     |     |     |     |     |
| --- | --- | ------------- | --- | --- | ------------- | --- | ------------- | --- | --- | --- | --- | --- |
(dB)
|     | A   |     | 0,0 f     |     | 0,0 f     |     | 0,0 f     |     | +0,25   |     | +0,07 / f |     |
| --- | --- | --- | --------- | --- | --------- | --- | --------- | --- | ------- | --- | --------- | --- |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | B   |     | 0,0 f     |     | 0,0 f     |     | 0,0 f     |     | -0,25   |     | -0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | C   |     | 0,2 f     |     | 0,2 f     |     | 0,2 f     |     | +0,25   |     | +0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | D   |     | 0,2 f     |     | 0,2 f     |     | 0,2 f     |     | -0,40   |     | -0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | E   |     | 0,4 f     |     | 0,4 f     |     | 0,4 f     |     | +0,25   |     | +0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | F   |     | 0,4 f N   |     | 0,4 f N   |     | 0,4 f N   |     | -0,40   |     | -0,07 / f | N   |
|     | G   |     | 0,8 f     |     | 0,86f     |     | 0,89 f    |     | +0,15   |     | +0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | H   |     | 0,8 f     |     | 0,86 f    |     | 0,89 f    |     | -1,10   |     | -0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | I   |     | 0,9 f     |     | 0,93 f    |     | 0,94 f    |     | -0,50   |     | +0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | J   |     | 1,0 f N   |     | 1,0 f N   |     | 1,0 f N   |     | -2,00   |     | +0,07 / f | N   |
|     | K   |     | 1,0 f     |     | 1,0 f     |     | 1,0 f     |     | -4,00   |     | -0,07 / f |     |
|     |     |     | N         |     | N         |     | N         |     |         |     |           | N   |
|     | L   |     | 1,2 f     |     | 1,13 f    |     | 1,11 f    |     | -8,00   |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |
|     | M   |     | 1,2 f     |     | 1,13 f    |     | 1,11 f    |     | -11,00  |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |
|     | N   |     | 1,8 f     |     | 1,60 f    |     | 1,5 f     |     | -35,00  |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |
|     | P   |     | 1,4 f     |     | 1,30 f    |     | 1,23 f    |     | -16,00  |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |
|     | Q   |     | 1,6 f     |     | 1,45 f    |     | 1,4 f     |     | -24,00  |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |
|     | S   |     | 2,12 f    |     | 1,83 f    |     | 1,7 f     |     | -40,00  |     |           | -   |
|     |     |     | N         |     | N         |     | N         |     |         |     |           |     |

ETSI

|     |     |     |     |     |     | 39  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |
Annex B (normative):
Addresses of parity bit accumulators for nldpc = 64 800
Example of interpretation of table B.4.
p = p ⊕i   p = p ⊕i   p = p ⊕i   p = p ⊕i    p = p ⊕i   p = p ⊕i
54 54 0 9318 9318 0 14392 14392 0 27561 27561 0 26909 26909 0 10219 10219 0
| p = p | ⊕i     |   p = | p ⊕i   |     |     |     |     |     |     |      |     |     |
| ----- | ------ | ----- | ------ | --- | --- | --- | --- | --- | --- | ---- | --- | --- |
| 2534  | 2534 0 | 8597  | 8597 0 |     |     |     |     |     |     |      |     |     |
| =     | ⊕i     | =     | ⊕i     | =   | ⊕i  |     | =   | ⊕i  | =   | ⊕i   | =   | ⊕i  |
| p p   |   p    | p     |    p   |     | p   |   p | p   |   p | p   |    p | p   |     |
144 144 1 9408 9408 1 14482 14482 1 27651 27651 1 26999 26999 1 10309 10309 1
| =        | ⊕i       | =     | ⊕i     |      |     |      |     |     |        |     |       |     |
| -------- | -------- | ----- | ------ | ---- | --- | ---- | --- | --- | ------ | --- | ----- | --- |
| p p      |   p      |       | p      |      |     |      |     |     |        |     |       |     |
| 2624     | 2624 1   | 8687  | 8687 1 |      |     |      |     |     |        |     |       |     |
| :  :  :  | :  :  :  | :  :  | :      |      |     |      |     |     |        |     |       |     |
| :  :  :  | :  :  :  | :  :  | :      |      |     |      |     |     |        |     |       |     |
| p =      | p ⊕i     |   p   | = p ⊕i |    p | =   | p ⊕i |   p | = p | ⊕i   p | = p | ⊕i    |     |
32364 32364 359 9228 9228 359 14302 14302 359 27471 27471 359 26819 26819 359
| p =   | p ⊕i  |   p      | = p ⊕i |    p | = p  | ⊕i       |     |     |     |     |     |     |
| ----- | ----- | -------- | ------ | ---- | ---- | -------- | --- | --- | --- | --- | --- | --- |
| 10129 | 10129 | 359 2444 | 2444   | 359  | 8507 | 8507 359 |     |     |     |     |     |     |

| =   | ⊕i  | =   | ⊕i  | =   | ⊕i  |     | =   | ⊕i  | =     | ⊕i  |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ----- | --- | --- | --- |
| p p |   p | p   |     | p   | p   |   p | p   |     |   p p |     |     |     |
55 55 360 7263 7263 360 4635 4635 360 2530 2530 360 28130 28130 360
| =        | ⊕i       |       | = ⊕i  |      | =    | ⊕i       |     |     |     |     |     |     |
| -------- | -------- | ----- | ----- | ---- | ---- | -------- | --- | --- | --- | --- | --- | --- |
| p p      |          |   p   | p     |    p | p    |          |     |     |     |     |     |     |
| 3033     | 3033 360 | 23830 | 23830 | 360  | 3651 | 3651 360 |     |     |     |     |     |     |
| :  :  :  | :  :  :  | :  :  | :     |      |      |          |     |     |     |     |     |     |
| :  :  :  | :  :  :  | :  :  | :     |      |      |          |     |     |     |     |     |     |
| :  :  :  | :  :  :  | :  :  | :     |      |      |          |     |     |     |     |     |     |
ETSI

40 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table B.1: Rate 1/4 (n = 64 800)
ldpc
23606 36098 1140 28859 18148 18510 6226 540 42014 20879 23802 47088
16419 24928 16609 17248 7693 24997 42587 16858 34921 21042 37024 20692
1874 40094 18704 14474 14004 11519 13106 28826 38669 22363 30255 31105
22254 40564 22645 22532 6134 9176 39998 23892 8937 15608 16854 31009
8037 40401 13550 19526 41902 28782 13304 32796 24679 27140 45980 10021
40540 44498 13911 22435 32701 18405 39929 25521 12497 9851 39223 34823
15233 45333 5041 44979 45710 42150 19416 1892 23121 15860 8832 10308
10468 44296 3611 1480 37581 32254 13817 6883 32892 40258 46538 11940
6705 21634 28150 43757 895 6547 20970 28914 30117 25736 41734 11392
22002 5739 27210 27828 34192 37992 10915 6998 3824 42130 4494 35739
8515 1191 13642 30950 25943 12673 16726 34261 31828 3340 8747 39225
18979 17058 43130 4246 4793 44030 19454 29511 47929 15174 24333 19354
16694 8381 29642 46516 32224 26344 9405 18292 12437 27316 35466 41992
15642 5871 46489 26723 23396 7257 8974 3156 37420 44823 35423 13541
42858 32008 41282 38773 26570 2702 27260 46974 1469 20887 27426 38553
22152 24261 8297
19347 9978 27802
34991 6354 33561
29782 30875 29523
9278 48512 14349
38061 4165 43878
8548 33172 34410
22535 28811 23950
20439 4027 24186
38618 8187 30947
35538 43880 21459
7091 45616 15063
5505 9315 21908
36046 32914 11836
7304 39782 33721
16905 29962 12980
11171 23709 22460
34541 9937 44500
14035 47316 8815
15057 45482 24461
30518 36877 879
7583 13364 24332
448 27056 4682
12083 31378 21670
1159 18031 2221
17028 38715 9350
17343 24530 29574
46128 31039 32818
20373 36967 18345
46685 20622 32806
ETSI

41 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table B.2: Rate 1/3 (n = 64 800)
ldpc
34903 20927 32093 1052 25611 16093 16454 5520 506 37399 18518 21120
11636 14594 22158 14763 15333 6838 22222 37856 14985 31041 18704
32910 17449 1665 35639 16624 12867 12449 10241 11650 25622 34372
19878 26894
29235 19780 36056 20129 20029 5457 8157 35554 21237 7943 13873
14980
9912 7143 35911 12043 17360 37253 25588 11827 29152 21936 24125
40870 40701 36035 39556 12366 19946 29072 16365 35495 22686 11106
8756 34863
19165 15702 13536 40238 4465 40034 40590 37540 17162 1712 20577
14138
31338 19342 9301 39375 3211 1316 33409 28670 12282 6118 29236 35787
11504 30506 19558 5100 24188 24738 30397 33775 9699 6215 3397 37451
34689 23126 7571 1058 12127 27518 23064 11265 14867 30451 28289
2966
11660 15334 16867 15160 38343 3778 4265 39139 17293 26229 42604
13486
31497 1365 14828 7453 26350 41346 28643 23421 8354 16255 11055
24279
15687 12467 13906 5215 41328 23755 20800 6447 7970 2803 33262 39843
5363 22469 38091 28457 36696 34471 23619 2404 24229 41754 1297
18563
3673 39070 14480 30279 37483 7580 29519 30519 39831 20252 18132
20010
34386 7252 27526 12950 6875 43020 31566 39069 18985 15541 40020
16715
1721 37332 39953 17430 32134 29162 10490 12971 28581 29331 6489
35383
736 7022 42349 8783 6767 11871 21675 10325 11548 25978 431 24085
1925 10602 28585 12170 15156 34404 8351 13273 20208 5800 15367
21764
16279 37832 34792 21250 34192 7406 41488 18346 29227 26127 25493
7048
39948 28229 24899
17408 14274 38993
38774 15968 28459
41404 27249 27425
41229 6082 43114
13957 4979 40654
3093 3438 34992
34082 6172 28760
42210 34141 41021
14705 17783 10134
41755 39884 22773
14615 15593 1642
29111 37061 39860
9579 33552 633
12951 21137 39608
38244 27361 29417
2939 10172 36479
29094 5357 19224
9562 24436 28637
40177 2326 13504
6834 21583 42516
40651 42810 25709
31557 32138 38142
18624 41867 39296
37560 14295 16245
6821 21679 31570
25339 25083 22081
8047 697 35268
9884 17073 19995
26848 35245 8390
18658 16134 14807
12201 32944 5035
25236 1216 38986
42994 24782 8681
28321 4932 34249
4107 29382 32124
22157 2624 14468
38788 27081 7936
4368 26148 10578
25353 4122 39751
ETSI

42 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table B.3: Rate 2/5 (n = 64 800)
ldpc
31413 18834 28884 947 23050 14484 14809 4968 455 33659 16666 19008 25796 31795 12152 28229 31684 30160
13172 19939 13354 13719 6132 20086 34040 13442 27958 16813 29619 16553 12184 35088 31226 15293 8483 28002
1499 32075 14962 11578 11204 9217 10485 23062 30936 17892 24204 24885 38263 33386 24892 14880 13334 12584
32490 18086 18007 4957 7285 32073 19038 7152 12486 13483 24808 21759 23114 37995 29796 28646 2558 19687
32321 10839 15620 33521 23030 10646 26236 19744 21713 36784 8016 12869 34336 10551 36245 6259 4499 26336
35597 11129 17948 26160 14729 31943 20416 10000 7882 31380 27858 33356 35407 175 7203 11952 28386 8405
14125 12131 36199 4058 35992 36594 33698 15475 1566 18498 12725 7067 14654 38201 22605 10609 961 7582
17406 8372 35437 2888 1184 30068 25802 11056 5507 26313 32205 37232 28404 6595 1018 10423 13191 26818
15254 5365 17308 22519 35009 718 5240 16778 23131 24092 20587 33385 19932 3524 29305 15922 36654 21450
27455 17602 4590 21767 22266 27357 30400 8732 5596 3060 33703 3596 31749 20247 8128 10492 1532 1205
6882 873 10997 24738 20770 10067 13379 27409 25463 2673 6998 31378 18026 36357 26735 30551 36482 22153
15181 13645 34501 3393 3840 35227 15562 23615 38342 12139 19471 15483 7543 29767 13588 5156 11330 34243
13350 6707 23709 37204 25778 21082 7511 14588 10010 21854 28375 33591 13333 25965 8463 28616 35369 13322
12514 4695 37190 21379 18723 5802 7182 2529 29936 35860 28338 10835 14504 36796 19710 8962 1485 21186
34283 25610 33026 31017 21259 2165 21807 37578 1175 16710 21939 30841 4528 25299 7318 23541 17445 35561
27292 33730 6836 26476 27539 35784 18245 16394 17939 23094 19216 17432 35091 25550 14798 33133 11593 19895
11655 6183 38708 28408 35157 17089 13998 36029 15052 16617 5638 36464 7824 215 1248 33917 7863 33651
15693 28923 26245 9432 11675 25720 26405 5838 31851 26898 8090 37037 30848 5362 17291 20063 28331 10702
24418 27583 7959 35562 37771 17784 11382 11156 37855 7073 21685 34515 28932 30249 27073 13195 21107 21859
10977 13633 30969 7516 11943 18199 5231 13825 19589 23661 11150 35602 13062 2103 16206 4364 31137 4804
19124 30774 6670 37344 16510 26317 23518 22957 6348 34069 8845 20175 7129 32062 19612 5585 2037 4830
34985 14441 25668 4116 3019 21049 37308 24551 24727 20104 24850 12114 9512 21936 38833 30672 16927 14800
38187 28527 13108 13985 1425 21477 30807 8613 26241 33368 35913 32477 35849 33754 23450
5903 34390 24641 26556 23007 27305 38247 2621 9122 32806 21554 18685 18705 28656 18111
17287 27292 19033 22749 27456 32187
ETSI

|     | 43  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Table B.4: Rate 1/2 (n  = 64 800)
ldpc
| 54 9318 14392 27561 26909 10219 2534  8 28158 8069          |     |     |
| ----------------------------------------------------------- | --- | --- |
| 8597   9 16583 11098                                        |     |     |
| 55 7263 4635 2530 28130 3033 23830 3651   10 16681 28363    |     |     |
| 56 24731 23583 26036 17299 5750 792 9169   11 13980 24725   |     |     |
| 57 5811 26154 18653 11551 15447 13685  12 32169 17989       |     |     |
| 16264   13 10907 2767                                       |     |     |
| 58 12610 11347 28768 2792 3174 29371  14 21557 3818         |     |     |
| 12997   15 26676 12422                                      |     |     |
| 59 16789 16018 21449 6165 21202 15850  16 7676 8754         |     |     |
| 3186   17 14905 20232                                       |     |     |
| 60 31016 21449 17618 6213 12166 8334  18 15719 24646        |     |     |
| 18212   19 31942 8589                                       |     |     |
| 61 22836 14213 11327 5896 718 11727 9308   20 19978 27197   |     |     |
| 62 2091 24941 29966 23634 9013 15587  21 27060 15071        |     |     |
| 5444   22 6071 26649                                        |     |     |
| 63 22207 3983 16904 28534 21415 27524  23 10393 11176       |     |     |
| 25912   24 9597 13370                                       |     |     |
| 64 25687 4501 22193 14665 14798 16158  25 7081 17677        |     |     |
| 5491   26 1433 19513                                        |     |     |
27 26925 9014
65 4520 17094 23397 4264 22370 16941
| 21526   28 19202 8900                                      |     |     |
| ---------------------------------------------------------- | --- | --- |
| 66 10490 6182 32370 9597 30841 25954  29 18152 30647       |     |     |
| 2762   30 20803 1737                                       |     |     |
| 67 22120 22865 29870 15147 13668 14955  31 11804 25221     |     |     |
| 19235   32 31683 17783                                     |     |     |
| 68 6689 18408 18346 9918 25746 5443  33 29694 9345         |     |     |
| 20645   34 12280 26611                                     |     |     |
| 69 29982 12529 13858 4746 30370 10023  35 6526 26122       |     |     |
| 24828   36 26165 11241                                     |     |     |
| 70 1262 28032 29888 13063 24033 21951  37 7666 26962       |     |     |
| 7863   38 16290 8480                                       |     |     |
| 71 6594 29642 31451 14831 9509 9335  39 11774 10120        |     |     |
| 31552   40 30051 30426                                     |     |     |
| 72 1358 6454 16633 20354 24598 624 5265   41 1335 15424    |     |     |
| 73 19529 295 18011 3080 13364 8032 15323   42 6865 17742   |     |     |
| 74 11981 1510 7960 21462 9129 11370  43 31779 12489        |     |     |
| 25741   44 32120 21001                                     |     |     |
| 75 9276 29656 4543 30699 20646 21921  45 14508 6996        |     |     |
| 28050   46 979 25024                                       |     |     |
| 76 15975 25634 5520 31119 13715 21949  47 4554 21896       |     |     |
| 19605   48 7989 21777                                      |     |     |
| 77 18688 4608 31755 30165 13103 10706  49 4972 20661       |     |     |
| 29224   50 6612 2730                                       |     |     |
| 78 21514 23117 12245 26035 31656 25631  51 12742 4418      |     |     |
52 29194 595
30699
| 79 9674 24966 31285 29908 17042 24588  53 19267 20113   |     |     |
| ------------------------------------------------------- | --- | --- |
| 31857                                                   |     |     |
80 21856 27777 29919 27000 14897 11409
7122
81 29773 23310 263 4877 28622 20545
22092
82 15605 5651 21864 3967 14419 22757
15896
83 30145 1759 10139 29223 26086 10556
5098
84 18815 16575 2936 24457 26738 6030 505
85 30326 22298 27562 20131 26390 6247
24791
86 928 29246 21246 12400 15311 32309
18608
87 20314 6025 26689 16302 2296 3244
19613
88 6237 11943 22851 15642 23857 15112
20947
89 26403 25168 19038 18384 8882 12719
7093
0 14567 24965
1 3908 100
2 10279 240
3 24102 764
4 12383 4173
5 13861 15918
6 21327 1046
7 5288 14579
ETSI

44 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table B.5: Rate 3/5 (n = 64 800)
ldpc
36 25012 13944
37 22513 6687
22422 10282 11626 19997 11161 2922 3122 99 5625 17064 8270 179 38 4934 12587
25087 16218 17015 828 20041 25656 4186 11629 22599 17305 22515 6463 39 21197 5133
11049 22853 25706 14388 5500 19245 8732 2177 13555 11346 17265 3069 40 22705 6938
16581 22225 12563 19717 23577 11555 25496 6853 25403 5218 15925 21766 41 7534 24633
16529 14487 7643 10715 17442 11119 5679 14155 24213 21000 1116 15620 42 24400 12797
5340 8636 16693 1434 5635 6516 9482 20189 1066 15013 25361 14243 43 21911 25712
18506 22236 20912 8952 5421 15691 6126 21595 500 6904 13059 6802 44 12039 1140
8433 4694 5524 14216 3685 19721 25420 9937 23813 9047 25651 16826 45 24306 1021
21500 24814 6344 17382 7064 13929 4004 16552 12818 8720 5286 2206 46 14012 20747
22517 2429 19065 2921 21611 1873 7507 5661 23006 23128 20543 19777 47 11265 15219
1770 4636 20900 14931 9247 12340 11008 12966 4471 2731 16445 791 48 4670 15531
6635 14556 18865 22421 22124 12697 9803 25485 7744 18254 11313 9004 49 9417 14359
19982 23963 18912 7206 12500 4382 20067 6177 21007 1195 23547 24837 50 2415 6504
756 11158 14646 20534 3647 17728 11676 11843 12937 4402 8261 22944 51 24964 24690
9306 24009 10012 11081 3746 24325 8060 19826 842 8836 2898 5019 52 14443 8816
7575 7455 25244 4736 14400 22981 5543 8006 24203 13053 1120 5128 53 6926 1291
3482 9270 13059 15825 7453 23747 3656 24585 16542 17507 22462 14670 54 6209 20806
15627 15290 4198 22748 5842 13395 23918 16985 14929 3726 25350 24157 55 13915 4079
24896 16365 16423 13461 16615 8107 24741 3604 25904 8716 9604 20365 56 24410 13196
3729 17245 18448 9862 20831 25326 20517 24618 13282 5099 14183 8804 57 13505 6117
16455 17646 15376 18194 25528 1777 6066 21855 14372 12517 4488 17490 58 9869 8220
1400 8135 23375 20879 8476 4084 12936 25536 22309 16582 6402 24360 59 1570 6044
25119 23586 128 4761 10443 22536 8607 9752 25446 15053 1856 4040 60 25780 17387
377 21160 13474 5451 17170 5938 10256 11972 24210 17833 22047 16108 61 20671 24913
13075 9648 24546 13150 23867 7309 19798 2988 16858 4825 23950 15125 62 24558 20591
20526 3553 11525 23366 2452 17626 19265 20172 18060 24593 13255 1552 63 12402 3702
18839 21132 20119 15214 14705 7096 10174 5663 18651 19700 12524 14033 64 8314 1357
4127 2971 17499 16287 22368 21463 7943 18880 5567 8047 23363 6797 65 20071 14616
10651 24471 14325 4081 7258 4949 7044 1078 797 22910 20474 4318 66 17014 3688
21374 13231 22985 5056 3821 23718 14178 9978 19030 23594 8895 25358 67 19837 946
6199 22056 7749 13310 3999 23697 16445 22636 5225 22437 24153 9442 68 15195 12136
7978 12177 2893 20778 3175 8645 11863 24623 10311 25767 17057 3691 69 7758 22808
20473 11294 9914 22815 2574 8439 3699 5431 24840 21908 16088 18244 70 3564 2925
8208 5755 19059 8541 24924 6454 11234 10492 16406 10831 11436 9649 71 3434 7769
16264 11275 24953 2347 12667 19190 7257 7174 24819 2938 2522 11749
3627 5969 13862 1538 23176 6353 2855 17720 2472 7428 573 15036
0 18539 18661
1 10502 3002
2 9368 10761
3 12299 7828
4 15048 13362
5 18444 24640
6 20775 19175
7 18970 10971
8 5329 19982
9 11296 18655
10 15046 20659
11 7300 22140
12 22029 14477
13 11129 742
14 13254 13813
15 19234 13273
16 6079 21122
17 22782 5828
18 19775 4247
19 1660 19413
20 4403 3649
21 13371 25851
22 22770 21784
23 10757 14131
24 16071 21617
25 6393 3725
26 597 19968
27 5743 8084
28 6770 9548
29 4285 17542
30 13568 22599
31 1786 4617
32 23238 11648
33 19627 2030
34 13601 13458
35 13740 17328
ETSI

|     | 45  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Table B.6: Rate 2/3 (n  = 64 800)
ldpc
0 10491 16043 506 12826 8065 8226 2767 240 18673 9279 10579 20928   14 8002 18591
1 17819 8313 6433 6224 5120 5824 12812 17187 9940 13447 13825 18483   15 14742 14089
2 17957 6024 8681 18628 12794 5915 14576 10970 12064 20437 4455 7151   16 253 3045
3 19777 6183 9972 14536 8182 17749 11341 5556 4379 17434 15477 18532   17 1274 19286
4 4651 19689 1608 659 16707 14335 6143 3058 14618 17894 20684 5306   18 14777 2044
5 9778 2552 12096 12369 15198 16890 4851 3109 1700 18725 1997 15882   19 13920 9900
6 486 6111 13743 11537 5591 7433 15227 14145 1483 3887 17431 12430   20 452 7374
7 20647 14311 11734 4180 8110 5525 12141 15761 18661 18441 10569 8192   21 18206 9921
8 3791 14759 15264 19918 10132 9062 10010 12786 10675 9682 19246 5454   22 6131 5414
9 19525 9485 7777 19999 8378 9209 3163 20232 6690 16518 716 7353   23 10077 9726
10 4588 6709 20202 10905 915 4317 11073 13576 16433 368 3508 21171   24 12045 5479
11 14072 4033 19959 12608 631 19494 14160 8249 10223 21504 12395 4322   25 4322 7990
| 12 13800 14161   |     | 26 15616 5550    |
| ---------------- | --- | ---------------- |
| 13 2948 9647     |     | 27 15561 10661   |
| 14 14693 16027   |     | 28 20718 7387    |
| 15 20506 11082   |     | 29 2518 18804    |
| 16 1143 9020     |     | 30 8984 2600     |
| 17 13501 4014    |     | 31 6516 17909    |
| 18 1548 2190     |     | 32 11148 98      |
33 20559 3704
19 12216 21556
| 20 2095 19897    |     | 34 7510 1569     |
| ---------------- | --- | ---------------- |
| 21 4189 7958     |     | 35 16000 11692   |
| 22 15940 10048   |     | 36 9147 10303    |
| 23 515 12614     |     | 37 16650 191     |
| 24 8501 8450     |     | 38 15577 18685   |
| 25 17595 16784   |     | 39 17167 20917   |
| 26 5913 8495     |     | 40 4256 3391     |
| 27 16394 10423   |     | 41 20092 17219   |
| 28 7409 6981     |     | 42 9218 5056     |
| 29 6678 15939    |     | 43 18429 8472    |
| 30 20344 12987   |     | 44 12093 20753   |
| 31 2510 14588    |     | 45 16345 12748   |
| 32 17918 6655    |     | 46 16023 11095   |
| 33 6703 19451    |     | 47 5048 17595    |
| 34 496 4217      |     | 48 18995 4817    |
| 35 7290 5766     |     | 49 16483 3536    |
| 36 10521 8925    |     | 50 1439 16148    |
| 37 20379 11905   |     | 51 3661 3039     |
| 38 4090 5838     |     | 52 19010 18121   |
| 39 19082 17040   |     | 53 8968 11793    |
| 40 20233 12352   |     | 54 13427 18003   |
| 41 19365 19546   |     | 55 5303 3083     |
| 42 6249 19030    |     | 56 531 16668     |
| 43 11037 19193   |     | 57 4771 6722     |
58 5695 7960
44 19760 11772
| 45 19644 7428   |     | 59 3589 14630   |
| --------------- | --- | --------------- |
| 46 16076 3521   |     |                 |
47 11779 21062
48 13062 9682
49 8934 5217
50 11087 3319
51 18892 4356
52 7894 3898
53 5963 4360
54 7346 11726
55 5182 5609
56 2412 17295
57 9845 20494
58 6687 1864
59 20564 5216
0 18226 17207
1 9380 8266
2 7073 3065
3 18252 13437
4 9161 15642
5 10714 10153
6 11585 9078
7 5359 9418
8 9024 9515
9 1206 16354
10 14994 1102
11 9375 20796
12 15964 6027
13 14789 6452
ETSI

|     | 46  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Table B.7: Rate 3/4 (n  = 64 800)
ldpc
0 6385 7901 14611 13389 11200 3252 5243 2504 2722 821 7374   29 4655 14128
1 11359 2698 357 13824 12772 7244 6752 15310 852 2001 11417   30 9584 13123
2 7862 7977 6321 13612 12197 14449 15137 13860 1708 6399 13444   31 13987 9597
3 1560 11804 6975 13292 3646 3812 8772 7306 5795 14327 7866   32 15409 12110
4 7626 11407 14599 9689 1628 2113 10809 9283 1230 15241 4870   33 8754 15490
5 1610 5699 15876 9446 12515 1400 6303 5411 14181 13925 7358   34 7416 15325
6 4059 8836 3405 7853 7992 15336 5970 10368 10278 9675 4651   35 2909 15549
7 4441 3963 9153 2109 12683 7459 12030 12221 629 15212 406   36 2995 8257
8 6007 8411 5771 3497 543 14202 875 9186 6235 13908 3563   37 9406 4791
9 3232 6625 4795 546 9781 2071 7312 3399 7250 4932 12652   38 11111 4854
10 8820 10088 11090 7069 6585 13134 10158 7183 488 7455 9238   39 2812 8521
11 1903 10818 119 215 7558 11046 10615 11545 14784 7961 15619   40 8476 14717
12 3655 8736 4917 15874 5129 2134 15944 14768 7150 2692 1469   41 7820 15360
13 8316 3820 505 8923 6757 806 7957 4216 15589 13244 2622   42 1179 7939
14 14463 4852 15733 3041 11193 12860 13673 8152 6551 15108 8758   43 2357 8678
| 15 3149 11981    | 44 7703 6216   |     |
| ---------------- | -------------- | --- |
| 16 13416 6906    | 0 3477 7067    |     |
| 17 13098 13352   | 1 3931 13845   |     |
| 18 2009 14460    | 2 7675 12899   |     |
3 1754 8187
19 7207 4314
| 20 3312 3945     | 4 7785 1400     |     |
| ---------------- | --------------- | --- |
| 21 4418 6248     | 5 9213 5891     |     |
| 22 2669 13975    | 6 2494 7703     |     |
| 23 7571 9023     | 7 2576 7902     |     |
| 24 14172 2967    | 8 4821 15682    |     |
| 25 7271 7138     | 9 10426 11935   |     |
| 26 6135 13670    | 10 1810 904     |     |
| 27 7490 14559    | 11 11332 9264   |     |
| 28 8657 2466     | 12 11312 3570   |     |
| 29 8599 12834    | 13 14916 2650   |     |
| 30 3470 3152     | 14 7679 7842    |     |
| 31 13917 4365    | 15 6089 13084   |     |
| 32 6024 13730    | 16 3938 2751    |     |
| 33 10973 14182   | 17 8509 4648    |     |
| 34 2464 13167    | 18 12204 8917   |     |
| 35 5281 15049    | 19 5749 12443   |     |
| 36 1103 1849     | 20 12613 4431   |     |
| 37 2058 1069     | 21 1344 4014    |     |
| 38 9654 6095     | 22 8488 13850   |     |
| 39 14311 7667    | 23 1730 14896   |     |
| 40 15617 8146    | 24 14942 7126   |     |
| 41 4588 11218    | 25 14983 8863   |     |
| 42 13660 6243    | 26 6578 8564    |     |
| 43 8578 7874     | 27 4947 396     |     |
28 297 12805
44 11741 2686
| 0 1022 1264     | 29 13878 6692    |     |
| --------------- | ---------------- | --- |
| 1 12604 9965    | 30 11857 11186   |     |
| 2 8217 2707     | 31 14395 11493   |     |
| 3 3156 11793    | 32 16145 12251   |     |
| 4 354 1514      | 33 13462 7428    |     |
| 5 6978 14058    | 34 14526 13119   |     |
| 6 7922 16079    | 35 2535 11243    |     |
| 7 15087 12138   | 36 6465 12690    |     |
| 8 5053 6470     | 37 6872 9334     |     |
| 9 12687 14932   | 38 15371 14023   |     |
| 10 15458 1763   | 39 8101 10187    |     |
| 11 8121 1721    | 40 11963 4848    |     |
| 12 12431 549    | 41 15125 6119    |     |
| 13 4129 7091    | 42 8051 14465    |     |
| 14 1426 8415    | 43 11139 5167    |     |
| 15 9783 7604    | 44 2883 14521    |     |
| 16 6295 11329   |                  |     |
17 1409 12061
18 8065 9087
19 2918 8438
20 1293 14115
21 3922 13851
22 3851 4000
23 5865 1768
24 2655 14957
25 5565 6332
26 4303 12631
27 11653 12236
28 16025 7632
ETSI

|     | 47  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Table B.8: Rate 4/5 (n  = 64 800)
ldpc
| 0 149 11212 5575 6360 12559 8108 8505 408 10026 12828     | 1 4219 1870     |     |
| --------------------------------------------------------- | --------------- | --- |
| 1 5237 490 10677 4998 3869 3734 3092 3509 7703 10305      | 2 10968 8054    |     |
| 2 8742 5553 2820 7085 12116 10485 564 7795 2972 2157      | 3 6970 5447     |     |
| 3 2699 4304 8350 712 2841 3250 4731 10105 517 7516        | 4 3217 5638     |     |
| 4 12067 1351 11992 12191 11267 5161 537 6166 4246 2363    | 5 8972 669      |     |
| 5 6828 7107 2127 3724 5743 11040 10756 4073 1011 3422     | 6 5618 12472    |     |
| 6 11259 1216 9526 1466 10816 940 3744 2815 11506 11573    | 7 1457 1280     |     |
| 7 4549 11507 1118 1274 11751 5207 7854 12803 4047 6484    | 8 8868 3883     |     |
| 8 8430 4115 9440 413 4455 2262 7915 12402 8579 7052       | 9 8866 1224     |     |
| 9 3885 9126 5665 4505 2343 253 4707 3742 4166 1556        | 10 8371 5972    |     |
| 10 1704 8936 6775 8639 8179 7954 8234 7850 8883 8713      | 11 266 4405     |     |
| 11 11716 4344 9087 11264 2274 8832 9147 11930 6054 5455   | 12 3706 3244    |     |
| 12 7323 3970 10329 2170 8262 3854 2087 12899 9497 11700   | 13 6039 5844    |     |
| 13 4418 1467 2490 5841 817 11453 533 11217 11962 5251     | 14 7200 3283    |     |
| 14 1541 4525 7976 3457 9536 7725 3788 2982 6307 5997      | 15 1502 11282   |     |
| 15 11484 2739 4023 12107 6516 551 2572 6628 8150 9852     | 16 12318 2202   |     |
| 16 6070 1761 4627 6534 7913 3730 11866 1813 12306 8249    | 17 4523 965     |     |
| 17 12441 5489 8748 7837 7660 2102 11341 2936 6712 11977   | 18 9587 7011    |     |
| 18 10155 4210                                             | 19 2552 2051    |     |
20 12045 10306
19 1010 10483
| 20 8900 10250    | 21 11070 5104   |     |
| ---------------- | --------------- | --- |
| 21 10243 12278   | 22 6627 6906    |     |
| 22 7070 4397     | 23 9889 2121    |     |
| 23 12271 3887    | 24 829 9701     |     |
| 24 11980 6836    | 25 2201 1819    |     |
| 25 9514 4356     | 26 6689 12925   |     |
| 26 7137 10281    | 27 2139 8757    |     |
| 27 11881 2526    | 28 12004 5948   |     |
| 28 1969 11477    | 29 8704 3191    |     |
| 29 3044 10921    | 30 8171 10933   |     |
| 30 2236 8724     | 31 6297 7116    |     |
| 31 9104 6340     | 32 616 7146     |     |
| 32 7342 8582     | 33 5142 9761    |     |
| 33 11675 10405   | 34 10377 8138   |     |
| 34 6467 12775    | 35 7616 5811    |     |
| 35 3186 12198    | 0 7285 9863     |     |
| 0 9621 11445     | 1 7764 10867    |     |
| 1 7486 5611      | 2 12343 9019    |     |
| 2 4319 4879      | 3 4414 8331     |     |
| 3 2196 344       | 4 3464 642      |     |
| 4 7527 6650      | 5 6960 2039     |     |
| 5 10693 2440     | 6 786 3021      |     |
| 6 6755 2706      | 7 710 2086      |     |
| 7 5144 5998      | 8 7423 5601     |     |
9 8120 4885
8 11043 8033
| 9 4846 4435     | 10 12385 11990   |     |
| --------------- | ---------------- | --- |
| 10 4157 9228    | 11 9739 10034    |     |
| 11 12270 6562   | 12 424 10162     |     |
| 12 11954 7592   | 13 1347 7597     |     |
| 13 7420 2592    | 14 1450 112      |     |
| 14 8810 9636    | 15 7965 8478     |     |
|                 | 16 8945 7397     |     |
| 15 689 5430     | 17 6590 8316     |     |
| 16 920 1304     | 18 6838 9011     |     |
| 17 1253 11934   | 19 6174 9410     |     |
| 18 9559 6016    | 20 255 113       |     |
| 19 312 7589     | 21 6197 5835     |     |
| 20 4439 4197    | 22 12902 3844    |     |
| 21 4002 9555    | 23 4377 3505     |     |
| 22 12232 7779   | 24 5478 8672     |     |
| 23 1494 8782    | 25 4453 2132     |     |
| 24 10749 3969   | 26 9724 1380     |     |
| 25 4368 3479    | 27 12131 11526   |     |
| 26 6316 5342    | 28 12323 9511    |     |
| 27 2455 3493    | 29 8231 1752     |     |
| 28 12157 7405   | 30 497 9022      |     |
| 29 6598 11495   | 31 9288 3080     |     |
| 30 11805 4455   | 32 2481 7515     |     |
| 31 9625 2090    | 33 2696 268      |     |
| 32 4731 2321    | 34 4023 12341    |     |
| 33 3578 2608    | 35 7108 5553     |     |
34 8504 1849
35 4027 1151
0 5647 4935
ETSI

|     | 48  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Table B.9: Rate 5/6 (n  = 64 800)
ldpc
0 4362 416 8909 4156 3216 3112 2560 2912 6405 8593 4969 6723   15 9027 3415
1 2479 1786 8978 3011 4339 9313 6397 2957 7288 5484 6031 10217   16 1690 3866
2 10175 9009 9889 3091 4985 7267 4092 8874 5671 2777 2189 8716   17 2854 8469
3 9052 4795 3924 3370 10058 1128 9996 10165 9360 4297 434 5138   18 6206 630
4 2379 7834 4835 2327 9843 804 329 8353 7167 3070 1528 7311   19 363 5453
5 3435 7871 348 3693 1876 6585 10340 7144 5870 2084 4052 2780   20 4125 7008
6 3917 3111 3476 1304 10331 5939 5199 1611 1991 699 8316 9960   21 1612 6702
7 6883 3237 1717 10752 7891 9764 4745 3888 10009 4176 4614 1567   22 9069 9226
8 10587 2195 1689 2968 5420 2580 2883 6496 111 6023 1024 4449   23 5767 4060
9 3786 8593 2074 3321 5057 1450 3840 5444 6572 3094 9892 1512   24 3743 9237
10 8548 1848 10372 4585 7313 6536 6379 1766 9462 2456 5606 9975   25 7018 5572
11 8204 10593 7935 3636 3882 394 5968 8561 2395 7289 9267 9978   26 8892 4536
12 7795 74 1633 9542 6867 7352 6417 7568 10623 725 2531 9115   27 853 6064
13 7151 2482 4260 5003 10105 7419 9203 6691 8798 2092 8263 3755   28 8069 5893
14 3600 570 4527 200 9718 6771 1995 8902 5446 768 1103 6520   29 2051 2885
| 15 6304 7621    |     | 0 10691 3153   |
| --------------- | --- | -------------- |
| 16 6498 9209    |     | 1 3602 4055    |
| 17 7293 6786    |     | 2 328 1717     |
| 18 5950 1708    |     | 3 2219 9299    |
| 19 8521 1793    |     | 4 1939 7898    |
| 20 6174 7854    |     | 5 617 206      |
| 21 9773 1190    |     | 6 8544 1374    |
| 22 9517 10268   |     | 7 10676 3240   |
| 23 2181 9349    |     | 8 6672 9489    |
9 3170 7457
24 1949 5560
| 25 1556 555     |     | 10 7868 5731    |
| --------------- | --- | --------------- |
| 26 8600 3827    |     | 11 6121 10732   |
| 27 5072 1057    |     | 12 4843 9132    |
| 28 7928 3542    |     | 13 580 9591     |
| 29 3226 3762    |     | 14 6267 9290    |
| 0 7045 2420     |     | 15 3009 2268    |
| 1 9645 2641     |     | 16 195 2419     |
| 2 2774 2452     |     | 17 8016 1557    |
| 3 5331 2031     |     | 18 1516 9195    |
| 4 9400 7503     |     | 19 8062 9064    |
| 5 1850 2338     |     | 20 2095 8968    |
| 6 10456 9774    |     | 21 753 7326     |
| 7 1692 9276     |     | 22 6291 3833    |
| 8 10037 4038    |     | 23 2614 7844    |
| 9 3964 338      |     | 24 2303 646     |
| 10 2640 5087    |     | 25 2075 611     |
| 11 858 3473     |     | 26 4687 362     |
| 12 5582 5683    |     | 27 8684 9940    |
| 13 9523 916     |     | 28 4830 2065    |
| 14 4107 1559    |     | 29 7038 1363    |
| 15 4506 3491    |     | 0 1769 7837     |
| 16 8191 4182    |     | 1 3801 1689     |
| 17 10192 6157   |     | 2 10070 2359    |
| 18 5668 3305    |     | 3 3667 9918     |
4 1914 6920
19 3449 1540
| 20 4766 2697    |     | 5 4244 5669     |
| --------------- | --- | --------------- |
| 21 4069 6675    |     | 6 10245 7821    |
| 22 1117 1016    |     | 7 7648 3944     |
| 23 5619 3085    |     | 8 3310 5488     |
| 24 8483 8400    |     | 9 6346 9666     |
| 25 8255 394     |     | 10 7088 6122    |
| 26 6338 5042    |     | 11 1291 7827    |
| 27 6174 5119    |     | 12 10592 8945   |
| 28 7203 1989    |     | 13 3609 7120    |
| 29 1781 5174    |     | 14 9168 9112    |
| 0 1464 3559     |     | 15 6203 8052    |
| 1 3376 4214     |     | 16 3330 2895    |
| 2 7238 67       |     | 17 4264 10563   |
| 3 10595 8831    |     | 18 10556 6496   |
| 4 1221 6513     |     | 19 8807 7645    |
| 5 5300 4652     |     | 20 1999 4530    |
| 6 1429 9749     |     | 21 9202 6818    |
| 7 7878 5131     |     | 22 3403 1734    |
| 8 4435 10284    |     | 23 2106 9023    |
| 9 6331 5507     |     | 24 6881 3883    |
| 10 6662 4941    |     | 25 3895 2171    |
| 11 9614 10238   |     | 26 4062 6424    |
| 12 8400 8025    |     | 27 3755 9536    |
| 13 9156 5630    |     | 28 4683 2131    |
| 14 7067 8878    |     | 29 7347 8027    |
ETSI

|     |     | 49  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | ----------------------------------- |
Table B.10: Rate 8/9 (n  = 64 800)
ldpc
| 0 6235 2848     | 16 4698 2285   | 11 6627 6243   |     |
| --------------- | -------------- | -------------- | --- |
| 3222            | 17 4760 3917   | 12 2644 5073   |     |
| 1 5800 3492     | 18 1859 4058   | 13 4212 5088   |     |
| 5348            | 19 6141 3527   | 14 3463 3889   |     |
| 2 2757 927 90   | 0 2148 5066    | 15 5306 478    |     |
| 3 6961 4516     | 1 1306 145     | 16 4320 6121   |     |
| 4739            | 2 2319 871     | 17 3961 1125   |     |
| 4 1172 3237     | 3 3463 1061    | 18 5699 1195   |     |
| 6264            | 4 5554 6647    | 19 6511 792    |     |
| 5 1927 2425     | 5 5837 339     | 0 3934 2778    |     |
| 3683            | 6 5821 4932    | 1 3238 6587    |     |
| 6 3714 6309     | 7 6356 4756    | 2 1111 6596    |     |
| 2495            | 8 3930 418     | 3 1457 6226    |     |
| 7 3070 6342     | 9 211 3094     | 4 1446 3885    |     |
| 7154            | 10 1007 4928   | 5 3907 4043    |     |
| 8 2428 613      | 11 3584 1235   | 6 6839 2873    |     |
| 3761            | 12 6982 2869   | 7 1733 5615    |     |
| 9 2906 264      | 13 1612 1013   | 8 5202 4269    |     |
| 5927            | 14 953 4964    | 9 3024 4722    |     |
| 10 1716 1950    | 15 4555 4410   | 10 5445 6372   |     |
| 4273            | 16 4925 4842   | 11 370 1828    |     |
| 11 4613 6179    | 17 5778 600    | 12 4695 1600   |     |
| 3491            | 18 6509 2417   | 13 680 2074    |     |
| 12 4865 3286    | 19 1260 4903   | 14 1801 6690   |     |
0 3369 3031
| 6005          |                | 15 2669 1377   |     |
| ------------- | -------------- | -------------- | --- |
| 13 1343 5923  | 1 3557 3224    | 16 2463 1681   |     |
| 3529          | 2 3028 583     | 17 5972 5171   |     |
| 14 4589 4035  | 3 3258 440     | 18 5728 4284   |     |
| 2132          | 4 6226 6655    | 19 1696 1459   |     |
| 15 1579 3920  | 5 4895 1094    |                |     |
| 6737          | 6 1481 6847    |                |     |
| 16 1644 1191  | 7 4433 1932    |                |     |
| 5998          | 8 2107 1649    |                |     |
| 17 1482 2381  | 9 2119 2065    |                |     |
| 4620          | 10 4003 6388   |                |     |
| 18 6791 6014  | 11 6720 3622   |                |     |
| 6596          | 12 3694 4521   |                |     |
| 19 2738 5918  | 13 1164 7050   |                |     |
| 3786          | 14 1965 3613   |                |     |
| 0 5156 6166   | 15 4331 66     |                |     |
| 1 1504 4356   | 16 2970 1796   |                |     |
| 2 130 1904    | 17 4652 3218   |                |     |
| 3 6027 3187   | 18 1762 4777   |                |     |
| 4 6718 759    | 19 5736 1399   |                |     |
| 5 6240 2870   | 0 970 2572     |                |     |
| 6 2343 1311   | 1 2062 6599    |                |     |
| 7 1039 5465   | 2 4597 4870    |                |     |
| 8 6617 2513   | 3 1228 6913    |                |     |
| 9 1588 5222   | 4 4159 1037    |                |     |
5 2916 2362
10 6561 535
| 11 4765 2054   | 6 395 1226     |     |     |
| -------------- | -------------- | --- | --- |
| 12 5966 6892   | 7 6911 4548    |     |     |
| 13 1969 3869   | 8 4618 2241    |     |     |
| 14 3571 2420   | 9 4120 4280    |     |     |
| 15 4632 981    | 10 5825 474    |     |     |
| 16 3215 4163   | 11 2154 5558   |     |     |
| 17 973 3117    | 12 3793 5471   |     |     |
| 18 3802 6198   | 13 5707 1595   |     |     |
| 19 3794 3948   | 14 1403 325    |     |     |
| 0 3196 6126    | 15 6601 5183   |     |     |
| 1 573 1909     | 16 6369 4569   |     |     |
| 2 850 4034     | 17 4846 896    |     |     |
| 3 5622 1601    | 18 7092 6184   |     |     |
| 4 6005 524     | 19 6764 7127   |     |     |
| 5 5251 5783    | 0 6358 1951    |     |     |
| 6 172 2032     | 1 3117 6960    |     |     |
| 7 1875 2475    | 2 2710 7062    |     |     |
| 8 497 1291     | 3 1133 3604    |     |     |
| 9 2566 3430    | 4 3694 657     |     |     |
| 10 1249 740    | 5 1355 110     |     |     |
| 11 2944 1948   | 6 3329 6736    |     |     |
| 12 6528 2899   | 7 2505 3407    |     |     |
| 13 2243 3616   | 8 2462 4806    |     |     |
| 14 867 3733    | 9 4216 214     |     |     |
| 15 1374 4702   | 10 5348 5619   |     |     |
ETSI

|     |     | 50  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | ----------------------------------- |
Table B.11: Rate 9/10 (n  = 64 800)
ldpc
| 0 5611 2563 2900    | 2 4433 4361    | 4 5155 3858    |     |
| ------------------- | -------------- | -------------- | --- |
| 1 5220 3143 4813    | 3 5198 541     | 5 1517 1312    |     |
| 2 2481 834 81       | 4 1146 4426    | 6 2554 3158    |     |
| 3 6265 4064 4265    | 5 3202 2902    | 7 5280 2643    |     |
| 4 1055 2914 5638    | 6 2724 525     | 8 4990 1353    |     |
| 5 1734 2182 3315    | 7 1083 4124    | 9 5648 1170    |     |
| 6 3342 5678 2246    | 8 2326 6003    | 10 1152 4366   |     |
| 7 2185 552 3385     | 9 5605 5990    | 11 3561 5368   |     |
| 8 2615 236 5334     | 10 4376 1579   | 12 3581 1411   |     |
| 9 1546 1755 3846    | 11 4407 984    | 13 5647 4661   |     |
| 10 4154 5561 3142   | 12 1332 6163   | 14 1542 5401   |     |
| 11 4382 2957 5400   | 13 5359 3975   | 15 5078 2687   |     |
| 12 1209 5329 3179   | 14 1907 1854   | 16 316 1755    |     |
| 13 1421 3528 6063   | 15 3601 5748   | 17 3392 1991   |     |
| 14 1480 1072 5398   | 16 6056 3266   |                |     |
| 15 3843 1777 4369   | 17 3322 4085   |                |     |
| 16 1334 2145 4163   | 0 1768 3244    |                |     |
| 17 2368 5055 260    | 1 2149 144     |                |     |
| 0 6118 5405         | 2 1589 4291    |                |     |
3 5154 1252
1 2994 4370
| 2 3405 1669    | 4 1855 5939    |     |     |
| -------------- | -------------- | --- | --- |
| 3 4640 5550    | 5 4820 2706    |     |     |
| 4 1354 3921    | 6 1475 3360    |     |     |
| 5 117 1713     | 7 4266 693     |     |     |
| 6 5425 2866    | 8 4156 2018    |     |     |
| 7 6047 683     | 9 2103 752     |     |     |
| 8 5616 2582    | 10 3710 3853   |     |     |
| 9 2108 1179    | 11 5123 931    |     |     |
| 10 933 4921    | 12 6146 3323   |     |     |
| 11 5953 2261   | 13 1939 5002   |     |     |
| 12 1430 4699   | 14 5140 1437   |     |     |
| 13 5905 480    | 15 1263 293    |     |     |
| 14 4289 1846   | 16 5949 4665   |     |     |
| 15 5374 6208   | 17 4548 6380   |     |     |
| 16 1775 3476   | 0 3171 4690    |     |     |
| 17 3216 2178   | 1 5204 2114    |     |     |
| 0 4165 884     | 2 6384 5565    |     |     |
| 1 2896 3744    | 3 5722 1757    |     |     |
| 2 874 2801     | 4 2805 6264    |     |     |
| 3 3423 5579    | 5 1202 2616    |     |     |
| 4 3404 3552    | 6 1018 3244    |     |     |
| 5 2876 5515    | 7 4018 5289    |     |     |
| 6 516 1719     | 8 2257 3067    |     |     |
| 7 765 3631     | 9 2483 3073    |     |     |
10 1196 5329
8 5059 1441
| 9 5629 598     | 11 649 3918    |     |     |
| -------------- | -------------- | --- | --- |
| 10 5405 473    | 12 3791 4581   |     |     |
| 11 4724 5210   | 13 5028 3803   |     |     |
| 12 155 1832    | 14 3119 3506   |     |     |
| 13 1689 2229   | 15 4779 431    |     |     |
| 14 449 1164    | 16 3888 5510   |     |     |
| 15 2308 3088   | 17 4387 4084   |     |     |
| 16 1122 669    | 0 5836 1692    |     |     |
| 17 2268 5758   | 1 5126 1078    |     |     |
| 0 5878 2609    | 2 5721 6165    |     |     |
| 1 782 3359     | 3 3540 2499    |     |     |
| 2 1231 4231    | 4 2225 6348    |     |     |
| 3 4225 2052    | 5 1044 1484    |     |     |
| 4 4286 3517    | 6 6323 4042    |     |     |
| 5 5531 3184    | 7 1313 5603    |     |     |
| 6 1935 4560    | 8 1303 3496    |     |     |
| 7 1174 131     | 9 3516 3639    |     |     |
| 8 3115 956     | 10 5161 2293   |     |     |
| 9 3129 1088    | 11 4682 3845   |     |     |
| 10 5238 4440   | 12 3045 643    |     |     |
| 11 5722 4280   | 13 2818 2616   |     |     |
| 12 3540 375    | 14 3267 649    |     |     |
| 13 191 2782    | 15 6236 593    |     |     |
| 14 906 4432    | 16 646 2948    |     |     |
| 15 3225 1111   | 17 4213 1442   |     |     |
| 16 6296 2583   | 0 5779 1596    |     |     |
| 17 1457 903    | 1 2403 1237    |     |     |
| 0 855 4475     | 2 2217 1514    |     |     |
| 1 4097 3970    | 3 5609 716     |     |     |
ETSI

51 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex C (normative):
Addresses of parity bit accumulators for nldpc = 16 200
Table C.1: Rate 1/4 (n = 16 200)
ldpc
6295 9626 304 7695 4839 4936 1660 144 11203 5567 6347 12557
10691 4988 3859 3734 3071 3494 7687 10313 5964 8069 8296 11090
10774 3613 5208 11177 7676 3549 8746 6583 7239 12265 2674 4292
11869 3708 5981 8718 4908 10650 6805 3334 2627 10461 9285 11120
7844 3079 10773
3385 10854 5747
1360 12010 12202
6189 4241 2343
9840 12726 4977
Table C.2: Rate 1/3 (n = 16 200)
ldpc
416 8909 4156 3216 3112 2560 2912 6405 8593 4969 6723 6912
8978 3011 4339 9312 6396 2957 7288 5485 6031 10218 2226 3575
3383 10059 1114 10008 10147 9384 4290 434 5139 3536 1965 2291
2797 3693 7615 7077 743 1941 8716 6215 3840 5140 4582 5420
6110 8551 1515 7404 4879 4946 5383 1831 3441 9569 10472 4306
1505 5682 7778
7172 6830 6623
7281 3941 3505
10270 8669 914
3622 7563 9388
9930 5058 4554
4844 9609 2707
6883 3237 1714
4768 3878 10017
10127 3334 8267
Table C.3: Rate 2/5 (n = 16 200)
ldpc
5650 4143 8750 583 6720 8071 635 1767 1344 6922 738 6658
5696 1685 3207 415 7019 5023 5608 2605 857 6915 1770 8016
3992 771 2190 7258 8970 7792 1802 1866 6137 8841 886 1931
4108 3781 7577 6810 9322 8226 5396 5867 4428 8827 7766 2254
4247 888 4367 8821 9660 324 5864 4774 227 7889 6405 8963
9693 500 2520 2227 1811 9330 1928 5140 4030 4824 806 3134
1652 8171 1435
3366 6543 3745
9286 8509 4645
7397 5790 8972
6597 4422 1799
9276 4041 3847
8683 7378 4946
5348 1993 9186
6724 9015 5646
4502 4439 8474
5107 7342 9442
1387 8910 2660
Table C.4: Rate 1/2 (n = 16 200)
ldpc
20 712 2386 6354 4061 1062 5045 5158 12 3028 764
21 2543 5748 4822 2348 3089 6328 5876 13 5988 1057
22 926 5701 269 3693 2438 3190 3507 14 7411 3450
23 2802 4520 3577 5324 1091 4667 4449
24 5140 2003 1263 4742 6497 1185 6202
0 4046 6934
1 2855 66
2 6694 212
3 3439 1158
4 3850 4422
5 5924 290
6 1467 4049
7 7820 2242
8 4606 3080
9 4633 7877
10 3884 6868
11 8935 4996
ETSI

52 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.5: Rate 3/5 (n = 16 200)
ldpc
2765 5713 6426 3596 1374 4811 2182 544 3394 2840 4310 771
4951 211 2208 723 1246 2928 398 5739 265 5601 5993 2615
210 4730 5777 3096 4282 6238 4939 1119 6463 5298 6320 4016
4167 2063 4757 3157 5664 3956 6045 563 4284 2441 3412 6334
4201 2428 4474 59 1721 736 2997 428 3807 1513 4732 6195
2670 3081 5139 3736 1999 5889 4362 3806 4534 5409 6384 5809
5516 1622 2906 3285 1257 5797 3816 817 875 2311 3543 1205
4244 2184 5415 1705 5642 4886 2333 287 1848 1121 3595 6022
2142 2830 4069 5654 1295 2951 3919 1356 884 1786 396 4738
0 2161 2653
1 1380 1461
2 2502 3707
3 3971 1057
4 5985 6062
5 1733 6028
6 3786 1936
7 4292 956
8 5692 3417
9 266 4878
10 4913 3247
11 4763 3937
12 3590 2903
13 2566 4215
14 5208 4707
15 3940 3388
16 5109 4556
17 4908 4177
ETSI

53 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.6: Rate 2/3 (n
ldpc
= 16 200)
0 2084 1613 1548 1286 1460 3196 4297 2481 3369 3451 4620 2622
1 122 1516 3448 2880 1407 1847 3799 3529 373 971 4358 3108
2 259 3399 929 2650 864 3996 3833 107 5287 164 3125 2350
3 342 3529
4 4198 2147
5 1880 4836
6 3864 4910
7 243 1542
8 3011 1436
9 2167 2512
10 4606 1003
11 2835 705
12 3426 2365
13 3848 2474
14 1360 1743
0 163 2536
1 2583 1180
2 1542 509
3 4418 1005
4 5212 5117
5 2155 2922
6 347 2696
7 226 4296
8 1560 487
9 3926 1640
10 149 2928
11 2364 563
12 635 688
13 231 1684
14 1129 3894
ETSI

54 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.7: Rate 3/4 (n
ldpc
= 16 200)
3 3198 478 4207 1481 1009 2616 1924 3437 554 683 1801
4 2681 2135
5 3107 4027
6 2637 3373
7 3830 3449
8 4129 2060
9 4184 2742
10 3946 1070
11 2239 984
0 1458 3031
1 3003 1328
2 1137 1716
3 132 3725
4 1817 638
5 1774 3447
6 3632 1257
7 542 3694
8 1015 1945
9 1948 412
10 995 2238
11 4141 1907
0 2480 3079
1 3021 1088
2 713 1379
3 997 3903
4 2323 3361
5 1110 986
6 2532 142
7 1690 2405
8 1298 1881
9 615 174
10 1648 3112
11 1415 2808
ETSI

55 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.8: Rate 4/5 (n = 16 200)
ldpc
5 896 1565
6 2493 184
7 212 3210
8 727 1339
9 3428 612
0 2663 1947
1 230 2695
2 2025 2794
3 3039 283
4 862 2889
5 376 2110
6 2034 2286
7 951 2068
8 3108 3542
9 307 1421
0 2272 1197
1 1800 3280
2 331 2308
3 465 2552
4 1038 2479
5 1383 343
6 94 236
7 2619 121
8 1497 2774
9 2116 1855
0 722 1584
1 2767 1881
2 2701 1610
3 3283 1732
4 168 1099
5 3074 243
6 3460 945
7 2049 1746
8 566 1427
9 3545 1168
ETSI

56 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.9: Rate 5/6 (n = 16 200)
ldpc
3 2409 499 1481 908 559 716 1270 333 2508 2264 1702 2805
4 2447 1926
5 414 1224
6 2114 842
7 212 573
0 2383 2112
1 2286 2348
2 545 819
3 1264 143
4 1701 2258
5 964 166
6 114 2413
7 2243 81
0 1245 1581
1 775 169
2 1696 1104
3 1914 2831
4 532 1450
5 91 974
6 497 2228
7 2326 1579
0 2482 256
1 1117 1261
2 1257 1658
3 1478 1225
4 2511 980
5 2320 2675
6 435 1278
7 228 503
0 1885 2369
1 57 483
2 838 1050
3 1231 1990
4 1738 68
5 2392 951
6 163 645
7 2644 1704
ETSI

57 ETSI EN 302 307-1 V1.4.1 (2014-11)
Table C.10: Rate 8/9 (n
ldpc
= 16 200)
0 1558 712 805
1 1450 873 1337
2 1741 1129 1184
3 294 806 1566
4 482 605 923
0 926 1578
1 777 1374
2 608 151
3 1195 210
4 1484 692
0 427 488
1 828 1124
2 874 1366
3 1500 835
4 1496 502
0 1006 1701
1 1155 97
2 657 1403
3 1453 624
4 429 1495
0 809 385
1 367 151
2 1323 202
3 960 318
4 1451 1039
0 1098 1722
1 1015 1428
2 1261 1564
3 544 1190
4 1472 1246
0 508 630
1 421 1704
2 284 898
3 392 577
4 1155 556
0 631 1000
1 732 1368
2 1328 329
3 1515 506
4 1104 1172
ETSI

58 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex D (normative):
Additional Mode Adaptation and ACM tools
D.1 "ACM Command" signalling interface
"ACM Command" signalling input (see figure D.1) shall allow setting, by an external "transmission mode control unit",
of the transmission parameters to be adopted by the DVB-S2 modulator, for a specific portion of input data.
"ACM Command" shall carry the following information:
• MODCOD (5 bits, according to table 12).
• TYPE (2 bits, according to clause 5.5.2.3).
• CVALID (Command Valid).
• SEND (deliver Data).
The transmission format specified by MODCOD and TYPE shall be applied to user data received after
CVALID = active and before SEND = active. When SEND = active, the modulator shall deliver user data immediately,
even if a FECFRAME is not completed, by inserting the PADDING field (see clause 5.2.1). The user data included in
the interval between CVALID = active and SEND = active shall not exceed the capacity of (K -80) bits, K being
bch bch
the transmittable bits associated with a specific MODCOD and TYPE.
For input Transport Streams, ACM is implemented via null-packet deletion function, therefore input user data do not
correspond directly to the transmitted data. In this case, the SEND function is not relevant, and CVALID, MODCOD
and TYPE shall become active at least 10 times a second. The ACM modulator shall continuously apply the specified
MODCOD and TYPE to user data after CVALID = active.
An example temporization of ACM Command is given in figure D.1, using a single serial interface to convey
MODCOD, TYPE, CVALID(active = high-to-low transition) and SEND (active = low-to-high transition).
CK
IN
User
Data
ACM
COMMAND
MODCOD
CVALID
SEND
MODCOD(1) MODCOD(3) MODCOD(5) TYPE(2)
CVALID SEND
(high-to-low) MODCOD(2) MODCOD(4) TYPE(1) (low-to-high)
Figure D.1: Example temporization of ACM Command (serial format)
D.2 Input stream synchronizer
Delays and packet jitter introduced by DVB-S2 modems may depend from the transmitted bit-rate and may change in
time during ACM rate switching. The "Input Stream Synchronizer" (see figures D.1 and D.2) shall provide a
mechanism to regenerate, in the receiver, the clock of the Transport Stream (or Generic Packetized Stream) at the
modulator Mode Adapter input, in order to guarantee end-to-end constant bit rates and delays (see also
TR 102 376 [i.5]). Table D.1 indicates the applications in which the Input Stream Synchronizer is normative or
optional.
ETSI

59 ETSI EN 302 307-1 V1.4.1 (2014-11)
When ISSYI = 1 in MATYPE field (see table 3), a counter shall be activated (22 bits), clocked by the modulator symbol
rate (frequency R ). The Input Stream SYnchronization field (ISSY, 2 or 3 bytes) shall be appended after each input
s
packet (in the case of Transport Streams, before null-packet deletion takes place), as shown in figure D.2. ISSY shall be
coded according to table D.1, sending the following variables:
• ISCR (short: 15 bits; long: 22 bits) (ISCR = Input Stream Time Reference), loaded with the LSBs of the
counter content at the instant the relevant input packet is processed (at constant rate R ), and specifically the
IN
instant the MSB of the relevant packet arrives at the modulator input stream interface.
• BUFS (2+10 bits) (BUFS = maximum size of the requested receiver buffer to compensate delay variations). It
is assumed that a receiver FIFO buffer (see TR 102 376 [i.5]) operates on a single stream input
(i.e. corresponding to a specific MATYPE-2 configuration for SIS/MIS = 0 in MATYPE-1); the FIFO buffer
input is the recovered packet stream after FEC error correction, at the channel arriving rate, and after null
packet reinsertion, its output is the modulator output stream (to be sent to the TS demultiplexer in case of
Transport Stream), read with the recovered (transport) stream clock. If ISSYI = 1 and optional BUFS is used,
this variable shall be transmitted at least 5 times per second, replacing ISCR. The maximum buffer size
required in the receiver shall be 20 Mbits.
• BUFSTAT (2+10 bits) (BUFSTAT = actual status to reset the receiver buffer = number of filled bits). If
ISSYI = 1 and optional BUFSTAT is used, this variable shall be transmitted at least 5 times per second,
replacing ISCR. This value can be used to set the receiver buffer status during reception start-up procedure,
and to verify normal functioning in steady state.
Input Stream Synchroniser
Mod 222 R
s
Counter S Y UP Packetised
N C Input Stream
15 or 22 LSBs BUFSTAT
ISCR BUFS
S Y UP I S
CK IN N C S Y
Input
ISSY (2 or 3 bytes)
Packets Insertion after Packet
(optional)
Figure D.2: Input stream synchronizer block diagram
ETSI

|     |     |     | 60  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| --- | --- | --- | --- | ----------------------------------- | --- | --- |
Table D.1: ISSY field coding (2 or 3 bytes)
|     |     | First Byte  |     | Second Byte  | Third Byte  |     |
| --- | --- | ----------- | --- | ------------ | ----------- | --- |
bit-7 (MSB)  bit-6   bit-5 and bit-4  bit-3 and bit-2  bit-1 and bit-0  bit-7 to bit-0  bit-7 bit-0
0 = ISCR    MSB of  next 6 bits of ISCR   next 8 bits of  not present
| short |                     | short |     |                 |                 |     |
| ----- | ------------------- | ----- | --- | --------------- | --------------- | --- |
|       | ISCR                |       |     | SCR             |                 |     |
|       | short               |       |     | short           |                 |     |
| 1     | 0 =  6 MSBs of ISCR |       |     | next 8 bits of  | next 8 bits of  |     |
long
|     | ISCR   |     |     | ISCR   | ISCR   |     |
| --- | ------ | --- | --- | ------ | ------ | --- |
|     | long   |     |     | long   | long   |     |
1  1  00 = BUFS  BUFS unit  2 MSBs of BUFS  next 8 bits of BUFS not present
|     |     | 00 = bits   |     |     | when ISCR      | short   |
| --- | --- | ----------- | --- | --- | -------------- | ------- |
|     |     | 01 = Kbits  |     |     | is used; else  |         |
|     |     | 10 = Mbits  |     |     | reserved       |         |
11 = reserved
1  1  10 = BUFSTAT   BUFSTAT unit  2 MSBs of BUFSTAT  next 8 bits of  not present
|     |     | 00 = bits  |     | BUFSTAT  | when ISCR |     |
| --- | --- | ---------- | --- | -------- | --------- | --- |
short
01 = Kbits
is used; else
10 = Mbits
reserved
11 = reserved
1  1  others = reserved  reserved  reserved  reserved  not present
|     |     |     |     |     | when ISCR |     |
| --- | --- | --- | --- | --- | --------- | --- |
short
is used; else
reserved
NOTE:  For Generic Packetized Streams optional ISCR shall be limited to the "short" format.

An example receiver scheme to regenerate the output packet stream and the relevant clock R' IN  is given in
TR 102 376 [i.5].
| D.3  | Null-packet Deletion (normative for input transport  |     |     |     |     |     |
| ---- | ---------------------------------------------------- | --- | --- | --- | --- | --- |
streams and ACM)
Transport Stream rules require that the bit rates at the output of the MUX and the input of the DEMUX are constant in
time, and the end-to-end delay is also constant. In order to fulfil such requirements in an ACM environment, the
null-packet deletion function shall be activated (see TR 102 376 [i.5] for application examples).
As shown in figure D.3, Useful Packets (i.e. packets with PID≠8191
) (including the optional ISSY appended field)
D
shall be transmitted while null-packets (PID = 8191 ) (including the optional ISSY appended field) shall be removed.
D
After transmission of a UP, a counter called DNP (Deleted Null-Packets, 1 byte) shall be first reset and then
incremented at each deleted null-packet. The counter content shall be appended after the Least Significant Byte of the
next transmitted useful packet, then DNP shall be reset. When DNP reaches the maximum allowed value DNP = 255 ,
D
then if the following packet is again a null-packet this null-packet is kept as a useful packet and transmitted.
The resulting stream has UPL = (188 + 1) x 8 bits (for ISSYI = 0) or UPL = (188 + 2 + 1) x 8 bits (for ISSYI = 1
and ISCR ), or UPL = (188 + 3 + 1) x 8 bits (for ISSYI = 1 and ISCR ), since the Transport Stream packets are
short long
extended by the DNP and ISSY (optional) fields.
ETSI

|     |     |     |     |     | 61  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- | --- |

Reset after DNP
DNP
insertion
| Null-packet deletion  |     |     |     | Counter  |     |     |     |     |     |     |
| --------------------- | --- | --- | --- | -------- | --- | --- | --- | --- | --- | --- |

|     |     |     |     |     | Useful-  | DNP (1 byte)  |     |     |     |     |
| --- | --- | --- | --- | --- | -------- | ------------- | --- | --- | --- | --- |
Output
|     |     |        |     |     | packets  | Insertion after  |     |     |     |     |
| --- | --- | ------ | --- | --- | -------- | ---------------- | --- | --- | --- | --- |
|     |     | Input  |     |     |          |                  |     | ut  |     |     |
Next Useful
Null-
Packet
packets
Input
Optional
|         | UP     |       | UP     |         | Null-packet  |       | Null-packet  | I S   | UP  | I    |
| ------- | ------ | ----- | ------ | ------- | ------------ | ----- | ------------ | ----- | --- | ---- |
| S Y     |        | I S S |        | I S S Y |              | I S S |              | S Y   |     | S    |
| N       |        | S Y N |        | S N     |              | S     |              | S N   |     | S Y  |
| C       |        | Y  C  |        | Y  C    |              | Y  Y  |              | Y  C  |     |      |
|         | DNP=0  |       | DNP=0  |         | DNP=1        |       | DNP=2        |       |     |      |
|         |        |       | UP     | I D     | S UP         | I     | D            |       |     |      |
| Output  |        | S Y   |        | S N     | Y            | S     | N            |       |     |      |
|         |        | N     |        | S P     | N C          | S Y   | P            |       |     |      |
|         |        | C     |        | Y       |              |       |              |       |     |      |
Figure D.3: Null-packet deletion and DNP field (1 byte) insertion
| D.4  | BBHEADER and Merging/slicing Policy for various  |     |     |     |     |     |     |     |     |     |
| ---- | ------------------------------------------------ | --- | --- | --- | --- | --- | --- | --- | --- | --- |
application areas
According to the application area, BBHeader coding and Merging/slicing policy shall be according to table D.2.
ETSI

|     | 62  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --- | --- | --- | ----------------------------------- | --- |
Table D.2: BBHeader coding for various application areas and Merging/Slicing policy
Application  MATYPE-1  MATYPE-2  UPL  DFL  SYNC  SYNCD  CRC-8  Merging/
| area/configuration  |     |     |     | slicing  |
| ------------------- | --- | --- | --- | -------- |
policy
Broadcasting/CCM, single  111100Y  X  188 x8  K  -  47   Y  Y  Break
|      | D   | bch | HEX |             |
| ---- | --- | --- | --- | ----------- |
| TS   |     |     |     | No timeout  |
80 D
No Padding
No Dummy
Broadcasting, differentiated  1100Y0Y  Y  188 x8  K  -  47   Y  Y  Break
|                               | D           | bch  | HEX |             |
| ----------------------------- | ----------- | ---- | --- | ----------- |
| protection level per stream/  |             |      |     | Read (1)    |
|                               | (+16 or 24  | 80   |     |             |
| VCM, constant protection      |             | D    |     | No timeout  |
if
| level per TS, Multiple TS   | ISSYI = 1)  |     |     | No Padding  |
| --------------------------- | ----------- | --- | --- | ----------- |
Yes Dummy
DSNG with time variable  111011Y  X  189 x8+  K  -  47   Y  Y  Break
|                               | D           | bch  | HEX |            |
| ----------------------------- | ----------- | ---- | --- | ---------- |
| protection level/ACM, single  | (16 or 24)  | 80   |     | Read (0)   |
D
| TS input, NP- deletion, ACM  |     |     |     | No timeout  |
| ---------------------------- | --- | --- | --- | ----------- |
| Command active               |     |     |     | No Padding  |
Yes Dummy
Interactive services with  1100Y1Y  Y  189 x8  Y  47   Y  Y  Read(1) or
|                                | D           |       | HEX |              |
| ------------------------------ | ----------- | ----- | --- | ------------ |
| ACM over TS, differentiated    |             | ≤K -  |     | (2)          |
|                                | (+16 or 24  | bch   |     |              |
| protection per stream/ ACM,    |             |       |     | Yes Padding  |
|                                | if          | 80    |     |              |
| constant protection level per  |             | D     |     | Yes Dummy    |
ISSYI = 1)
| TS, Multiple TS, NP-  |     |     |     | YES         |
| --------------------- | --- | --- | --- | ----------- |
| deletion              |     |     |     | shortframe  |
(see note)
Interactive services (IP) with  010000Y  Y  0  Y  X  X  Y  Read(1) or
| ACM over GS, differentiated  |     | ≤K -  |     | (2)  |
| ---------------------------- | --- | ----- | --- | ---- |
bch
| protection per stream/ ACM,  |     |     |     | Yes Padding  |
| ---------------------------- | --- | --- | --- | ------------ |
80 D
| constant protection level per  |     |     |     | Yes Dummy   |
| ------------------------------ | --- | --- | --- | ----------- |
| input stream, Multiple         |     |     |     | YES         |
| Generic Stream                 |     |     |     | shortframe  |
(see note)
Interactive services (IP) with  011000Y  X  0  Y  X  X  Y  According to
| ACM over GS, time variable  |     | ≤K  |     | ACM  |
| --------------------------- | --- | --- | --- | ---- |
bch -
Command
| protection/ ACM, time  |     | 80   |     |              |
| ---------------------- | --- | ---- | --- | ------------ |
|                        |     | D    |     | Yes Padding  |
variable protection level,
Yes Dummy
Single Generic Stream, ACM
YES
Command active
shortframe
BC Broadcasting services  111100Y  X  188 D x8  K bch  -  47 HEX   Y  Y  Break
|     |     | 80   |     | No timeout  |
| --- | --- | ---- | --- | ----------- |
D
No Padding
No Dummy
X =   not defined; Y = according to configuration/computation Break = break packets in subsequent DATAFIELDs;
Timeout: maximum delay in merger/slicer buffer.
Read (0) = Read [K  (Normal FECFRAME) - 80] bits when available, otherwise dummy.
bch
Read (1) = Round-robin polling. Read [K  (Normal FECFRAME) - 80] bits from port i when available, otherwise poll the
bch
next port.
Read (2) = On timeout, read DFL bits from port i and select the shortest FECFRAME containing DFL.
NOTE:  Additional merging policy modes may be optionally implemented by manufacturers.

D.5  Signalling of reception quality via return channel
(Normative for ACM)
In ACM modes, the receiver shall signal the reception quality via an available return channel, according to the various
DVB interactive systems, such as for example DVB-RCS (EN 301 790 [6]), DVB-RCP (ETS 300 801 [7]), DVB-RCG
(EN 301 195 [8]), DVB-RCC (ES 200 800 [9]).
DVB "Network Independent Protocols for DVB Interactive Services" (ETS 300 802 [11]) may be adopted to achieve
maximum network interoperability. Other simpler or optimized solutions (e.g. to guarantee minimum signalling delay)
may be adopted to directly interface with the aforementioned DVB interactive systems.
ETSI

63 ETSI EN 302 307-1 V1.4.1 (2014-11)
The receiver shall evaluate quality-of-reception parameters, in particular carrier to noise plus interference ratio in dB
available at the receiver, indicated as CNI. CNI format shall be:
CNI = 20 + 10 {10 Log [C / (N + I)]} (positive integer, 8 bits, in the range 0 to 255).
10
In fact for DVB-S2 10 Log [C / (N + I)] may be in the range -2 dB to +23,5 dB.
10
10 Log [C / (N + I)] shall be evaluated with a quantized accuracy better than 1 dB (accuracy = mean error + 3 σ,
10
where σ is the standard deviation). Since modulation and coding modes for DVB-S2 are typically spaced 1 dB to 1,5 dB
apart, a quantized precision better than 0,3 dB is recommended in order to fully exploit system capabilities. The
measurement process is assumed to be continuous. A possible method to evaluate CNI is by using symbols known
a-priori at the receiver, such as those in the SOF field of the PLFRAME Header and, when available, pilot symbols.
CNI and other optional reception quality parameters (such as for example the BER on the channel evaluated by
counting the errors corrected by the LDPC decoder, the packet error rate detected by CRC-8, the CNI distance from the
QEF threshold) may optionally be used by the receiver to identify the maximum throughput DVB-S2 transmission
mode that it may decode at QEF, indicated by MODCOD_RQ (7 bits, b , ..., b ) where:
6 0
• (b , ..., b ) are coded according to MODCOD in table 12;
4 0
• b indicates the presence/absence of pilots: (b = 0 no pilots, b = 1 pilots);
5 5 5
• b = 1 indicates (b , ..., b ) are valid; b = 0 indicates (b , ..., b ) information is not available by the terminal.
6 5 0 6 5 0
As a minimum, the CNI and MODCOD_RQ parameters shall be sent to the satellite network operator Gateway every
time the protection on the DVB-S2 channel has to be changed. When no modification of the protection level is
requested, the optional message from the terminal to the Gateway shall indicate MODCOD_RQ = actual MODCOD
and pilot configuration of the frames received by the terminal. In specific applications, CNI and MODCOD_RQ fields
may be extended to an integer number of byte(s), by padding zeroes in MSB positions.
The maximum delay required for CNI and MODCOD evaluation and delivery to the Gateway via the interaction
channel shall be no more than 300 ms, but this delay should be minimized if services interruptions are to be avoided
under fast fading conditions (C/N+I variations as fast as 0,5 dB/s to 1 dB/s may occur in Ka band). Optionally the
gateway may acknowledge the reception of the message and the execution of the command by a message containing the
new adopted MODCOD, coded according to table 12. The allocated protection shall be equal or more robust than that
requested by the terminal.
Example Transmission Protocol using ETS 300 802 [11]
DVBS2_Change_Modcod message shall be sent from the receiving terminal to the satellite network operator gateway,
every time the protection on the DVB-S2 channel has to be changed.
DVBS2_Change_Modcod() length in bits (big-endian notation)
{
CNI; 8
MODCOD_RQ; 8
}
DVBS2_Ack_Modcod message shall optionally be sent from the Gateway to the receiving terminal to acknowledge the
DVB-S2 protection level modification. MODCOD_ACK shall be coded according to the MODCOD_RQ conventions.
DVBS2_Ack_Modcod() length in bits (big-endian notation)
{
MODCOD_ACK; 8
}
ETSI

64 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex E (normative):
SI and signal identification for DSNG and contribution
applications
In DSNG transmissions, editing of the SI tables in the field may be impossible due to operational problems. Therefore,
only the following MPEG.2-defined SI tables PAT, PMT and Transport Stream Descriptor Table (TSDT) are
mandatory. DSNG transmission using DVB-S2 shall implement SI according to annex D of EN 301 210 [3].
Satellite transmissions may be affected by interference problems, which may be generated by SNG stations not strictly
adhering to standard operating regulations. Although solutions to this problem are mostly based on operational rules,
DVB-S2 provides technical means to allow interfering station identification. DVB-S2 up-link stations (except stations
for broadcast services) shall make their signal identifiable by applying the Physical Layer Scrambling initialization
sequence n (n in the range 0 to 262 141; see clause 5.5.4) assigned to each station owner.
ETSI

|     | 65  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Annex F (normative):
Backwards Compatible modes (optional)
This annex F is intentionally left empty, since its content was considered obsolete.
ETSI

66 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex G (informative):
Supplementary information on receiver implementation
Receiver specification is not under the scope of the present document. Nevertheless the DVB-S2 specification has been
developed devoting a large effort to technical evaluations on the receiver design, in order to guarantee that the
end-to-end performance target may be met. Typical impairments that may significantly impact the performance of the
receiver are:
• phase noise of the LNB and tuner;
• quality of the transmitter and/or receiver oscillators;
• adjacent channel interference;
• satellite non-linearity.
The user guidelines document TR 102 376 [i.5] includes some tutorial material on receiver implementation, although
other techniques may be used offering the target functionalities and receiver performance.
G.1 Carrier recovery
This clause G.1 is intentionally left empty, please refer to TR 102 376 [i.5].
G.2 FEC decoding
This clause G.2 is intentionally left empty, please refer to TR 102 376 [i.5].
G.3 ACM: Transport Stream regeneration and clock
recovery using ISCR
This clause G.3 is intentionally left empty, please refer to TR 102 376 [i.5].
G.4 Non linearity pre-compensation and Intersymbol
Interference suppression techniques
This clause G.4 is intentionally left empty, please refer to TR 102 376 [i.5].
ETSI

67 ETSI EN 302 307-1 V1.4.1 (2014-11)
G.5 Interactive services using DVB-RCS return link: user
terminal synchronization
Interactive services can be operated with a DVB-RCS (EN 301 790 [6]) or DVB-RCS2 (TS 101 545-1 [13]) return path,
provided that an absolute time reference (NCR, Network Clock Reference) can be generated in the user terminal for
transmissions alignment. In DVB-RCS and DVB-RCS2 the hub broadcasts the NCR in the form of special transport
packets over the forward link. In case of DVB-S2 forward link, NCR is associated to the emission time, at the
transmitting side, of the first symbol of the SOF field.
In order to facilitate RCS/RCS2 synchronization at user terminal, a "SOF flag" output should be included in the
DVB-S2 receiver chipset. Furthermore, in order to allow alignment of the SOF flag with the relevant NCR, the receiver
chipset should implement an internal counter of the received physical layer frames (e.g. modulo M = 32), with arbitrary
start-up. The counter content should label both the "SOF flag" and the decoded data at the chip output. In practical
implementations the SOF flag label could be signalled serially on the SOF flag signal and the frame label on another
signal.
ETSI

68 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex H (informative):
Examples of possible use of the System
H.1 CCM digital TV broadcasting: bit rate capacity and
C/N requirements
This clause H.1 is intentionally left empty, please refer to TR 102 376 [i.5].
H.2 Distribution of multiple TS multiplexes to DTT
Transmitters (Multiple TS, CCM)
This clause H.2 is intentionally left empty, please refer to TR 102 376 [i.5].
H.3 SDTV and HDTV broadcasting with differentiated
protection (VCM, Multiple TS)
This clause H.3 is intentionally left empty, please refer to TR 102 376 [i.5].
H.4 DSNG Services using ACM (Single transport Stream,
information rate varying in time)
This clause H.4 is intentionally left empty, please refer to TR 102 376 [i.5].
H.5 IP Unicast Services (Non-uniform protection on a
user-by-user basis)
This clause H.5 is intentionally left empty, please refer to TR 102 376 [i.5].
H.6 Example performance of BC modes
This clause H.6 is intentionally left empty, please refer to TR 102 376 [i.5].
H.7 Satellite transponder models for simulations
For simulations, the "transparent" (i.e. non regenerative) satellite transponder model may be composed of an input filter
(IMUX), a power amplifier (TWT or SSA) and an output filter (OMUX). Two amplifier models are here defined, the
linearized TWTA (LTWTA) and the non-linearized TWTA. SSPAs have not been considered since they are less critical
than TWTAs in terms of degradations.
The reference symbol rate with the specified IMUX/OMUX filter bandwidth is Rs = 27,5 Mbaud.
ETSI

|     |     |     |     | 69  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --- | --- | --- | --- | --- | --- | ----------------------------------- | --- |

  SATELLITE TRANSPONDER MODEL
|     |       |     |        |     |     |     |     |
| --- | ----- | --- | ------ | --- | --- | --- | --- |
|     | IMUX  |     | POWER  |     |     |     |     |
OMUX
AMPLIFIER
Down-link
Noise

Figure H.1: Satellite transponder model
Figures H.2 and H.3 give the AM/AM and AM/PM TWTA characteristics.
Ku-band LTWTA Single Carrier Transfer Characteristics
(Measurement Frequency: 10992.5MHz)
|     | 0   |     |     |     |     |     | 30  |
| --- | --- | --- | --- | --- | --- | --- | --- |
-2
|     | -4  |     |     |     |     |     | 25 )geD( egnahC esahP tuptuO |
| --- | --- | --- | --- | --- | --- | --- | ---------------------------- |
-6
-8
20
)Bd( tuoP -10
-12
15
-14
-16
|     | -18 |     |     |     |     |     | 10  |
| --- | --- | --- | --- | --- | --- | --- | --- |
-20
5
-22
-24
|     | -26     |             |             |             |          |        | 0   |
| --- | ------- | ----------- | ----------- | ----------- | -------- | ------ | --- |
|     | -30 -28 | -26 -24 -22 | -20 -18 -16 | -14 -12 -10 | -8 -6 -4 | -2 0 2 | 4 6 |
Pin (dB)

Figure H.2: Linearized TWTA characteristic
ETSI

|     |     |     |     |     | 70  |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- |
Ka-band TWTA - Single Carrier Transfer Characteristics
|     |     | 0   |     |     |     |     |     | 70  |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     | -2  |     |     |     |     |     | 60  |     |
|     |     | -4  |     |     |     |     |     | 50  |     |
]Bd[ REWOP TUPTUO
|     |     | -6  |     |     |     |     |     | 40 ]geD[ esahP tuptuO |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --------------------- | --- |
|     |     | -8  |     |     |     |     |     | 30                    |     |
|     |     | -10 |     |     |     |     |     | 20                    |     |
|     |     | -12 |     |     |     |     |     | 10                    |     |
OUTPUT POWER
PHASE
|     |     | -14         |         |                  |       |       |     | 0   |     |
| --- | --- | ----------- | ------- | ---------------- | ----- | ----- | --- | --- | --- |
|     |     | -16         |         |                  |       |       |     | -10 |     |
|     |     | -20 -18 -16 | -14 -12 | -10              | -8 -6 | -4 -2 | 0 2 | 4 6 |     |
|     |     |             |         | INPUT POWER [dB] |       |       |     |     |     |
Figure H.3: Non-Linearized TWTA characteristic
|     |     |                       |     |     |     |                    |                       |     |                  |
| --- | --- | --------------------- | --- | --- | --- | ------------------ | --------------------- | --- | ---------------- |
|     |     | IMUX Ku-band (36 MHz) |     |     |     |                    | OMUX Ku-band (36 MHz) |     |                  |
|     | 0   |                       |     | 100 |     | 0                  |                       |     | 90               |
|     |     |                       |     | 90  |     | -5                 |                       |     | 80               |
|     | -10 |                       |     |     |     |                    |                       |     | )sn( yaleD puorG |
|     |     |                       |     | 80  |     | )Bd( noitcejeR -10 |                       |     | 70               |
70
|     | -20            |     |     |     |     | -15 |     |     | 60  |
| --- | -------------- | --- | --- | --- | --- | --- | --- | --- | --- |
|     | )Bd( noitcejeR |     |     | 60  |     |     |     |     |     |
|     | -30            |     |     |     |     | -20 |     |     | 50  |
50
|     |     |     |     |     |     | -25 |     |     | 40  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     | -40 |     |     | 40  |     |     |     |     |     |
|     |     |     |     |     |     | -30 |     |     | 30  |
30
|     | -50 |     |     | 20  |     | -35 |     |     | 20  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     | 10  |     | -40 |     |     | 10  |
-60
|     |     |         |       | 0   |     | -45 |     | 0   | 0   |
| --- | --- | ------- | ----- | --- | --- | --- | --- | --- | --- |
|     | -70 |         |       | -10 |     |     |     |     |     |
|     |     |         |       |     |     | -50 |     | 0   | 50  |
|     | -50 | -30 -10 | 10 30 | 50  |     |     |     |     |     |
Frequency (MHz)
Frequency (MHz)

Figure H.4: IMUX and OMUX characteristics
Other transponder bandwidths BW [MHz] may be obtained by scaling the IMUX and OMUX characteristics:
•  R(f) = Rejection [f ×(BW/36)].
•  G(f) = [(36/BW)] × Group-delay [f × (BW/36)].
The band-centre insertion loss is not indicated, but should be included in C  for link budget computation.
SAT
ETSI

|                                         |     |     |     | 71  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| --------------------------------------- | --- | --- | --- | --- | ----------------------------------- | --- |
| H.8  Phase noise masks for simulations  |     |     |     |     |                                     |     |
The following phase noise masks for consumer reception systems may be used to evaluate the carrier recovery
algorithms. The mask represents single side-band power spectral densities. The "aggregate" masks combine the phase
noise contributions of the LNB and of the relevant Tuner. Other sources of phase noise within the chain (e.g. satellite
transponder, up-link station, etc.) are usually negligible, and therefore the proposed masks may be considered as
representative of the full chain.
Table H.1: Aggregate Phase Noise masks for Simulation (in dBc/Hz)
⇒
|     | frequency              |   100 Hz  | 1 kHz  | 10 kHz  100 kHz  | 1 MHz  | > 10 MHz  |
| --- | ---------------------- | --------- | ------ | ---------------- | ------ | --------- |
|     | Aggregate1 (typical)   | -25       | -50    | -73  -93         | -103   | -114      |
|     | Aggregate2 (critical)  | -25       | -50    | -73  -85         | -103   | -114      |

ETSI

72 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex I (normative):
Mode Adaptation input interfaces (optional)
I.1 Mode Adaptation input interface with separate
signalling circuit (optional)
Mode Adaptation optional input interface (see figure 1) shall allow implementing the merging of multiple input streams
by an external "Mode Adaptation Unit", respecting all the rules of the DVB-S2 specification. To allow to vary the
transmission parameters to be adopted by the DVB-S2 modulator, it shall also transport the ACM command associated
to each specific Data Field.
According to figure 3 Mode Adaptation shall be a sequence of Data Fields (according to clause 5.1.5), where each
individual Data Field is preceded by a BBHEADER, according to clause 5.1.6 and to figure 3, and a Stream Adaptation
command (SA command), transporting the transmission parameters to be adopted by the DVB-S2 modulator for each
specific Data Field and corresponding BBHEADER.
"SA Command" (similar to the ACM command format, see clause D.1) shall carry the following information:
• MODCOD (5 bits, according to table 12).
• TYPE (2 bits, according to clause 5.5.2.3).
• CVALID (Command Valid).
• SEND (end of MA Packet).
The CVALID=active indicates the start of a MA Packet (MSB of the BB Header).
The transmission format specified by MODCOD and TYPE shall be applied to MA Packet received after
CVALID=active and before SEND=active. When SEND=active, the modulator shall deliver user data immediately,
even if a FECFRAME is not completed, by inserting the PADDING field (see clause 5.2.1). The user data included in
the interval between CVALID=active and SEND=active shall not exceed the capacity of (K -80) bits, K being the
bch bch
transmittable bits associated with a specific MODCOD and TYPE.
An example temporization of SA Command is given in figure I.1, using a single serial interface to convey MODCOD,
TYPE, CVALID(active= high-to-low transition) and SEND (active= low-to-high transition).
CK
IN
MA Packet
SA
COMMAND
CVALID
MODCOD
TYPE
SEND MODCOD(1) MODCOD(3) MODCOD(5) TYPE(2)
CVALID SEND
(high-to-low) MODCOD(2) MODCOD(4) TYPE(1) (low-to-high)
Figure I.1: Example temporization of SA Command (serial format)
ETSI

|      |                                               |     | 73  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| ---- | --------------------------------------------- | --- | --- | ----------------------------------- |
| I.2  | Mode Adaptation input interface with in-band  |     |     |                                     |
signalling (optional)
Alternatively to clause I.1, the SA command can be mapped into a Transport Header to be prepended to the data
generated by the external Mode Adaptation Unit. According to figure I.1, Mode Adaptation shall be a sequence of Data
Fields (according to clause 5.1.5), where each individual Data Field is preceded by a BBHEADER, according to
clause 5.1.6 and to figure 3, and a Transport Header.
The Transport Header shall consist of 2 bytes as illustrated in figure I.2 and defined in table I.1. The first byte identifies
the start of the Mode Adaptation packet and shall correspond to the sequence 0xB8. The second byte shall indicate the
ACM command, defining the dynamic transmission parameters (MODCOD, TYPE) for the BBFRAME, according to
table I.2.
The BBFRAME shall consist of a valid BBHEADER, followed by the payload with length DFL, without padding bytes.
Stream Adaptation shall synchronize to the baseband frames (using the 0xB8 syncmarker and the DFL field inside the
BBHEADER.

|     | TSHEADER  | BBHEADER = 10 Bytes  |     | PAYLOAD = DFL bytes  |
| --- | --------- | -------------------- | --- | -------------------- |
|     | 0xB8      | ACM                  |     |                      |
Transport Header : 2 Bytes

Figure I.2: Mode Adaptation format at the Mode Adaptation input interface
Table I.1: Transport Header format
|     | Byte                     | Contents  |                          | Purpose  |
| --- | ------------------------ | --------- | ------------------------ | -------- |
|     | Byte 0  0xB8 syncmarker  |           | For BBF synchronization  |          |
Byte 1  ACM command byte  Defines modcod, frametype and pilot insertion

Table I.2: ACM command byte definition (acm[0] is the least significant bit)
|     | Bit fields                                                                     |     | Description  |     |
| --- | ------------------------------------------------------------------------------ | --- | ------------ | --- |
|     | Acm[4:0]  MODCOD (as defined in table 12)                                      |     |              |     |
|     | Acm[5]  pilots configuration (0 = no pilots, 1 = pilots)                       |     |              |     |
|     | Acm[6]       FECFRAME sizes (0 = normal: 64 800 bits; 1 = short: 16 200 bits)  |     |              |     |
|     | Acm[7]       reserved bit (set to 0)                                           |     |              |     |
ETSI

74 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex J (informative):
Bibliography
R. De Gaudenzi, A. Guillen i Fabregas, A. Martinez Vicente, B. Ponticelli, "APSK Coded Modulation Schemes for
Nonlinear Satellite Channels with High Power and Spectral Efficiency", in the Proc. of the AIAA Satellite
Communication Systems Conference 2002, Montreal, Canada, May 2002, Paper # 1861.
U. Reimers, A. Morello, "DVB-S2, the second generation standard for satellite broadcasting and unicasting",
International Journal on Satellite Communication Networks, 2004; 22.
M. Eroz, F.-W. Sun and L.-N. Lee, "DVB-S2 Low Density Parity Check Codes with near Shannon Limit Performance",
International Journal on Satellite Communication Networks, 2004; 22.
E. Casini, R. De Gaudenzi, A. Ginesi, "DVB-S2 modem algorithms design and performance over typical satellite
channels", International Journal on Satellite Communication Networks, 2004; 22.
F.-W. Sun Y. Jiang and L.-N. Lee, "Frame synchronization and pilot structure for DVB-S2", International Journal on
Satellite Communication Networks, 2004; 22.
A. Morello, R. Rinaldo, M. Vazquez-Castro, "DVB-S2 ACM modes for IP and MPEG unicast applications",
International Journal on Satellite Communication Networks, 2004; 22.
E. Chen, J. L. Koslov, V. Mignone, J. Santoru, "DVB-S2 Backward-Compatible modes: a bridge between the present
and the future", International Journal on Satellite Communication Networks, 2004; 22.
CENELEC EN 50083-9: "Cable networks for television signals, sound signals and interactive services -
Part 9: Interfaces for CATV/SMATV headends and similar professional equipment for DVB/MPEG-2 transport
streams".
ETSI TBR 30 (1997): "Satellite Earth Stations and Systems (SES); Satellite News Gathering Transportable Earth
Stations (SNG TES) operating in the 11-12/13-14 GHz frequency bands".
ETSI ETS 300 327: "Satellite Earth Stations and Systems (SES); Satellite News Gathering (SNG) Transportable Earth
Stations (TES) (13-14/11-12 GHz)".
ETSI EN 300 673: "Electromagnetic compatibility and Radio spectrum Matters (ERM); ElectroMagnetic Compatibility
(EMC) standard for Very Small Aperture Terminal (VSAT), Satellite News Gathering (SNG), Satellite Interactive
Terminals (SIT) and Satellite User Terminals (SUT) Earth Stations operated in the frequency ranges between 4 GHz
and 30 GHz in the Fixed Satellite Service (FSS)".
ETSI

|     | 75  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Annex K:
For future use

ETSI

|     | 76  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | ----------------------------------- |
Annex L:
For future use

ETSI

77 ETSI EN 302 307-1 V1.4.1 (2014-11)
Annex M (normative):
Transmission format for wideband satellite transponders
using time-slicing (optional)
This annex specifies the optional transmission format for high symbol-rate satellite carriers for broadcasting,
professional and interactive services. This format may optionally be adopted for wideband satellite transponders (e.g.
200 MHz to 500 MHz), where the transmission of a single or few wide-band carriers is preferable to the transmission of
a multiplicity of narrow-band carriers, for power and efficiency optimization or other needs. This format is intended to
permit the operation of time-slicing receivers, which are characterized by realtime high-speed coherent-demodulation
and PL-Header processing capabilities, but FEC decoding speed significantly lower than that of the wideband carrier. In
order to allow such receivers to select and decode a specific stream carrying one or more service(s) within its
performance capabilities, while discarding the other streams and services of the wide-band carrier, the transmitter shall
map the input services into streams (identified by a specific Time Slice Number, TSN). Such streams shall be
transmitted in time-slices (i.e. bursts) suitably spaced in time. A time-slicing burst (identified by a specific TSN) shall
correspond to one PL-Frame.
The Time Slice Number TSN -8 bits- may optionally correspond to MATYPE2 ISI field in the BB-Header
(clause 5.1.6).
Service 1, 2 Service 8
Service 1, 2
TSN=1 TSN=2 TSN=5 TSN=1 TSN=4
Time
PL-Frame PL-Frame PL-Frame PL-Frame PL-Frame
Figure M.1: Example of time-sliced transmission
The receiver can select TSN=1 and decode Service 1 or Service 2, and discard other TSNs and associated services.
Depending on the applications, the time-sliced transmission may correspond to a periodic sequence of slices
(e.g. TSN=1, TSN=2,….TSN=20, TSN=1, TSN=2,…, TSN=20,…) or to a non-ordered sequence of slices (e.g. TSN=1,
TSN=22, TSN=4,…) which may be decided "on-the-fly" at the transmitting side, according to service/traffic needs.
This annex specifies physical layer signalling that shall be introduced in the transmitted waveform to allow receiver
configuration in time slicing modes. Algorithms to define the slicing sequence at the transmitter site are left open to
optimization according to use cases. Such algorithms shall satisfy the receiver capabilities as defined in clause M.1. As
an example, in broadcasting applications the total wideband symbol rate can be constantly assigned (in static mode) to
"virtual carriers" of equal or different capacity, using CCM per virtual carrier. In unicasting ACM applications, where
the slice structure should follow the traffic requirements, "on the fly" allocation of resources (in dynamic mode) may
offer the best efficiency and flexibility.
Upper layer signalling shall be according to EN 300 468 [12].
M.1 Definition of Time-slicing receiver
Time-slicing receivers are characterized by:
(i) real-time high-speed coherent demodulation and PL-Header processing capabilities, including continuous
PL-frame synchronization;
(ii) maximum average decoding speed at FECFRAME level R (e.g. R =100 Mbit/s);
FEC FEC
NOTE: R may be significantly lower than the wide-band carrier bit-rate.
FEC
(iii) a minimum Guard Time T in μsec which shall separate two adjacent slices received by the decoder (which
G
may be time variable, and can be better defined for different receiver classes).
ETSI

|      |     |                         |     |     |     | 78  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |
| ---- | --- | ----------------------- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- |
| M.2  |     | TIME SLICE MODE CODING  |     |     |     |     |     |     |                                     |     |
This mode shall comply with clauses 4 and 5, with the exception of the PL-Header structure of clauses 5.5.2 to 5.5.4
which shall be coded according to the following clauses.
| M.2.1  |     | PL signalling  |     |     |     |     |     |     |     |     |
| ------ | --- | -------------- | --- | --- | --- | --- | --- | --- | --- | --- |
The PLHEADER is intended for receiver synchronization and physical layer signalling.
NOTE 1:  After decoding the PLHEADER, the receiver knows the PLFRAME duration and structure, the
modulation and coding scheme of the XFECFRAME, the presence or absence of pilot symbols.
The PLHEADER shall be extended to two SLOTs of 90 symbols, and shall be composed of the following fields:
•
SOF (26 symbols), identifying the Start of Frame;
•
PLS code (154 symbols): PLS (Physical Layer Signalling) code shall be a constraint length k=5, rate 1/5
convolutional code (77,16), whose output bits (c ,c ,c ,…,c ) are repeated twice to produce the (154,16)
|     |     |     |     |     |     | 0 1 | 2 76 |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | ---- | --- | --- | --- |
codeword (c 0 ,c 0 , c 1 ,c 1 , c 2 ,c 2 ,…, c 76 ,c 76 ), described by the following generator polynomials:
| gi=(g | i,0 ,g i,1 ,g | i,2 ,g i,3 ,g i,4 | )   |     |     |     |     |     |     |     |
| ----- | ------------- | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- |
g0=(10101); g1=(10111); g2=(11011); g3=(11111); g4=(11001);
|     |     |     | S   | S   | S   |     | S   |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | 0   | 1   |     | 2   | 3   |     |     |     |
|     | g   |     |     |     |     |     |     |     |     |     |
|     | i,0 |     | g   | g   | g   |     | g   |     |     |     |
|     |     |     | i,1 | i,2 |     | i,3 | i,4 |     |     |     |
p
i
|     |     |     |     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | +   | +   | +   |     |     |     |     |     |
+
Figure M.2: Convolutional encoding scheme
To output only 77 coded bits instead of 80, the following bits shall be punctured:
If (u , u ,u ,u ,..,u ) are the information bits, then each information bit shall generate 5 parity bits (p ,p ,p ,…,p ). Then,
| 0                      | 1 2 | 3 15 |                               |     |                       |     |     |     |     | 0 1 2 4 |
| ---------------------- | --- | ---- | ----------------------------- | --- | --------------------- | --- | --- | --- | --- | ------- |
| for information bits u |     |      | ,u , and u , the parity bit p |     |  shall be punctured.  |     |     |     |     |         |
|                        |     |      | 3 8 13                        |     | 4                     |     |     |     |     |         |
"Tail biting" shall be used to complete the encoding process: depending on the input bits, the initial state shall be chosen
so that the initial and final states are the same. The encoder initial state shall thus be set as:
|     |     |     |     |     | S =u ; S | =u ; S | =u ; S | =u   |     |     |
| --- | --- | --- | --- | --- | -------- | ------ | ------ | ---- | --- | --- |
|     |     |     |     |     | 0 15     | 1 14   | 2 13 3 | 12   |     |     |
Tail bits shall not be transmitted.
NOTE 2:  The repetitive structure of the PLS code may be exploited in the receivers for differential detection
synchronization, in presence of frequency and phase errors.
The resulting 154 coded bits shall be scrambled with the following sequence:
1 0 1 1 1 1 0 0 0 0 0 1 0 0 0 1 1 0 0 0 0 0 0 0 1 0 1 0 0 1 0 1 0 1 0 1 0 0 0 1 0 1 1 1 0 1 1 1 0 1 0 1 1 1 0 0 1 0 0 1 1 1
0 1 1 0 0 0 0 1 0 1 1 0 0 0 1 1 1 1 1 1 0 1 1 0 1 0 1 1 0 0 1 1 0 1 1 0 1 1 1 0 0 0 0 1 1 1 0 0 0 1 1 0 1 0 1 0 0 1 1 1 1 1
0 0 0 1 0 0 0 0 1 1 0 0 1 0 1 0 0 0 0 0 0 1 1 1 1 0 1 1 1 1
The PLHEADER, represented by the binary sequence (y , y ,...y ), shall be modulated into 180 π/2BPSK symbols
|     |     |     |     |     |     | 1 2 | 180 |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
according to the rule:
  I.  = Q.  = (1/√2) (1-2y ), I.  = - Q.  = - (1/√2) (1-2y ) for i = 1, 2, ..., 90
|     |     | 2i-1 | 2i-1 | 2i-1 | 2i  | 2i  |     | 2i  |     |     |
| --- | --- | ---- | ---- | ---- | --- | --- | --- | --- | --- | --- |
ETSI

|        |     |            |     |     |     |     | 79  |     |     | ETSI EN 302 307-1 V1.4.1 (2014-11)  |     |     |
| ------ | --- | ---------- | --- | --- | --- | --- | --- | --- | --- | ----------------------------------- | --- | --- |
| M.2.2  |     | SOF field  |     |     |     |     |     |     |     |                                     |     |     |
SOF shall be coded according to clause 5.5.2.1.
| M.2.3  |     | MODCOD field                |     |     |     |     |     |     |     |     |     |     |
| ------ | --- | --------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| •      |  (u | , u ,u ,u ,..,u ) = MODCOD  |     |     |     |     |     |     |     |     |     |     |
|        | 0   | 1 2 3 5                     |     |     |     |     |     |     |     |     |     |     |
The MODCOD field shall be extended with respect to clause 5.5.2.2, in order to allow additional modulation and
coding configurations. The two MSB u0 and u1 shall be coded as follows:
| •   | u =0   |       |     |     |   modes according to table 12.  |     |     |     |     |     |     |     |
| --- | ------ | ----- | --- | --- | ------------------------------- | --- | --- | --- | --- | --- | --- | --- |
0
If u 0  =1, then (u 1 ,…, u 5 ) shall be encoded according to EN 302 307-2 [14], clause 5.5.2.2 and EN 302 307-2 [14],
tables 17a to 17c,  that define the extended MODCODE configurations.
| M.2.4  |     | TYPE field  |     |     |     |     |     |     |     |     |     |     |
| ------ | --- | ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
•
|     |  (u | , u ) = TYPE (according to clause 5.5.2.3).  |     |     |     |     |     |     |     |     |     |     |
| --- | --- | -------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
6 7
| M.2.5  |     | TSN code           |     |                                           |     |     |     |     |     |     |     |     |
| ------ | --- | ------------------ | --- | ----------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
| •      |  (u | , …, u )= TSN      |     | (may correspond to MATYPE-2 field, ISI).  |     |     |     |     |     |     |     |     |
|        | 8   | 15                 |     |                                           |     |     |     |     |     |     |     |     |
| M.3    |     | Phase noise masks  |     |                                           |     |     |     |     |     |     |     |     |
The following typical phase noise masks shall be taken into account for receiver implementation.
Table M.1: PROFILE "Ku- DTH" and "Ka-DTH", SSB (dBc/Hz)
⇒
frequency    100 Hz  1 kHz  10 kHz  100 kHz  1 MHz  10 MHz  > 50 MHz
|     |     | Aggregate1 (typical)  |     | -25  |     | -50  | -73  | -93  | -103  |     | -114  | -117  |
| --- | --- | --------------------- | --- | ---- | --- | ---- | ---- | ---- | ----- | --- | ----- | ----- |
Aggregate2 (critical)  -25  -50  -73  -85   -103  -114  -117

Table M.2: PROFILE "Ku- Non DTH", SSB (dBc/Hz)
⇒
frequency    10 Hz  100 Hz  1 kHz  10 kHz  100 kHz  1 MHz  10 MHz  > 50 MHz
|     | Aggregate  |     |     | -33  | -62  | -79  | -89  |     | -99  | -109  | -119  | -120  |
| --- | ---------- | --- | --- | ---- | ---- | ---- | ---- | --- | ---- | ----- | ----- | ----- |

Table M.3: PROFILE "Ka- Non DTH", SSB (dBc/Hz)
⇒
|     |            |     | 10 Hz  |     | 100 Hz  | 1 kHz  | 10 kHz  | 100 kHz  |     | 1 MHz  | 10 MHz  | > 50 MHz  |
| --- | ---------- | --- | ------ | --- | ------- | ------ | ------- | -------- | --- | ------ | ------- | --------- |
|     | frequency  |     |        |     |         |        |         |          |     |        |         |           |
Aggregate1 (typical)  -33  -62  -79  -89  -95  -106  -116  -118

ETSI

|     |     |     | 80  | ETSI EN 302 307-1 V1.4.1 (2014-11)  |
| --- | --- | --- | --- | ----------------------------------- |
History
Document history
| V1.1.1  | March 2005   | Publication as EN 302 307  |     |     |
| ------- | ------------ | -------------------------- | --- | --- |
| V1.1.2  | June 2006    | Publication as EN 302 307  |     |     |
| V1.2.1  | August 2009  | Publication as EN 302 307  |     |     |
| V1.3.1  | March 2013   | Publication as EN 302 307  |     |     |
V1.4.1  July 2014  EN Approval Procedure  AP 20141104:  2014-07-07 to 2014-11-04
| V1.4.1  | November 2014  | Publication  |     |     |
| ------- | -------------- | ------------ | --- | --- |

ETSI
