ETSI TS 102 773 V1.4.1 (2016-03)

TECHNICAL SPECIFICATION

Digital Video Broadcasting (DVB);
Modulator Interface (T2-MI) for a second generation digital
terrestrial television broadcasting system (DVB-T2)

2

ETSI TS 102 773 V1.4.1 (2016-03)

Reference
RTS/JTC-DVB-364

Keywords
digital, DVB, satellite, TV

ETSI

650 Route des Lucioles
F-06921 Sophia Antipolis Cedex - FRANCE

Tel.: +33 4 92 94 42 00   Fax: +33 4 93 65 47 16

Siret N° 348 623 562 00017 - NAF 742 C
Association à but non lucratif enregistrée à la
Sous-Préfecture de Grasse (06) N° 7803/88

Important notice

The present document can be downloaded from:
http://www.etsi.org/standards-search

The present document may be made available in electronic versions and/or in print. The content of any electronic and/or
print versions of the present document shall not be modified without the prior written authorization of ETSI. In case of any
existing or perceived difference in contents between such versions and/or in print, the only prevailing document is the
print of the Portable Document Format (PDF) version kept on a specific network drive within ETSI Secretariat.

Users of the present document should be aware that the document may be subject to revision or change of status.
Information on the current status of this and other ETSI documents is available at
https://portal.etsi.org/TB/ETSIDeliverableStatus.aspx

If you find errors in the present document, please send your comment to one of the following services:
https://portal.etsi.org/People/CommiteeSupportStaff.aspx

Copyright Notification

No part may be reproduced or utilized in any form or by any means, electronic or mechanical, including photocopying
and microfilm except as authorized by written permission of ETSI.
The content of the PDF version shall not be modified without the written authorization of ETSI.
The copyright and the foregoing restriction extend to reproduction in all media.

© European Telecommunications Standards Institute 2016.
© European Broadcasting Union 2016.
All rights reserved.

DECTTM, PLUGTESTSTM, UMTSTM and the ETSI logo are Trade Marks of ETSI registered for the benefit of its Members.
3GPPTM and LTE™ are Trade Marks of ETSI registered for the benefit of its Members and
of the 3GPP Organizational Partners.
GSM® and the GSM logo are Trade Marks registered and owned by the GSM Association.

ETSI

3

ETSI TS 102 773 V1.4.1 (2016-03)

1

2
2.1
2.2

3
3.1
3.2
3.3

4
4.1
4.2
4.3

Contents
Intellectual Property Rights ................................................................................................................................ 6
Foreword ............................................................................................................................................................. 6
Modal verbs terminology .................................................................................................................................... 6
Scope ........................................................................................................................................................ 7
References ................................................................................................................................................ 7
Normative references ......................................................................................................................................... 7
Informative references ........................................................................................................................................ 7
Definitions, symbols and abbreviations ................................................................................................... 8
Definitions .......................................................................................................................................................... 8
Symbols ............................................................................................................................................................ 10
Abbreviations ................................................................................................................................................... 10
General description................................................................................................................................. 12
System overview .............................................................................................................................................. 12
System architecture .......................................................................................................................................... 12
Protocol stack ................................................................................................................................................... 13
T2-MI packets ........................................................................................................................................ 13
Introduction ...................................................................................................................................................... 13
T2-MI packet definition ................................................................................................................................... 14
T2-MI payload definitions ................................................................................................................................ 15
Baseband Frame .......................................................................................................................................... 15
Auxiliary stream I/Q data ........................................................................................................................... 15
Arbitrary cell insertion ................................................................................................................................ 16
L1-current T2-MI packets ........................................................................................................................... 17
L1-future ..................................................................................................................................................... 18
P2 bias balancing cells ................................................................................................................................ 19
DVB-T2 timestamp..................................................................................................................................... 20
Introduction ........................................................................................................................................... 20
Null timestamp ...................................................................................................................................... 21
Individual addressing .................................................................................................................................. 21
Introduction ........................................................................................................................................... 21
Existing addressing functions................................................................................................................ 22
Addressing functions specific to DVB-T2 ............................................................................................ 22
Introduction ..................................................................................................................................... 22
ACE-PAPR function ....................................................................................................................... 22
MISO group function ...................................................................................................................... 23
TR-PAPR function .......................................................................................................................... 24
L1-ACE-PAPR function .................................................................................................................. 24
TX-SIG FEF Sequence Numbers function ...................................................................................... 25
TX-SIG aux stream transmitter ID function .................................................................................... 26
Frequency function................................................................................................................................ 26
FEF part: Null ............................................................................................................................................. 26
FEF part: I/Q data ....................................................................................................................................... 27
FEF part: composite .................................................................................................................................... 28
FEF sub-part ............................................................................................................................................... 29
Introduction ........................................................................................................................................... 29
FEF sub-part: Null ................................................................................................................................ 29
FEF sub-part: IQ ................................................................................................................................... 30
FEF sub-part: PRBS .............................................................................................................................. 30
FEF sub-part: TX-SIG FEF ................................................................................................................... 31
Generation of L1 signalling from the T2-MI packets ....................................................................................... 31
Transmission order of T2-MI packets .............................................................................................................. 32
Timing of T2-MI packet transmission .............................................................................................................. 33
Transport of T2-MI packets ................................................................................................................... 35
Introduction ...................................................................................................................................................... 35

5
5.0
5.1
5.2
5.2.1
5.2.2
5.2.3
5.2.4
5.2.5
5.2.6
5.2.7
5.2.7.0
5.2.7.1
5.2.8
5.2.8.0
5.2.8.1
5.2.8.2
5.2.8.2.0
5.2.8.2.1
5.2.8.2.2
5.2.8.2.3
5.2.8.2.4
5.2.8.2.5
5.2.8.2.6
5.2.8.2.7
5.2.9
5.2.10
5.2.11
5.2.12
5.2.12.0
5.2.12.1
5.2.12.2
5.2.12.3
5.2.12.4
5.3
5.4
5.5

6
6.0

ETSI

4

ETSI TS 102 773 V1.4.1 (2016-03)

6.1
6.1.0
6.1.1
6.2
6.2.0
6.2.1
6.2.2
6.2.3
6.2.4

Encapsulation of T2-MI packets in MPEG-2 TS ............................................................................................. 35
Introduction................................................................................................................................................. 35
Description .................................................................................................................................................. 35
Encapsulation of MPEG-2 TS in IP packets..................................................................................................... 36
Introduction................................................................................................................................................. 36
Setup Information ....................................................................................................................................... 36
Transport Protocols ..................................................................................................................................... 37
Session Initiation and Control ..................................................................................................................... 37
Network Requirements ............................................................................................................................... 37

Annex A (normative):

Calculation of the CRC word ....................................................................... 38

T2 Modulator Information Packet (T2-MIP) .............................................. 39
Annex B (normative):
B.1  Use of the T2-MIP for over the air synchronization .............................................................................. 39
B.2  T2-MIP Definition .................................................................................................................................. 40
Field description ............................................................................................................................................... 40
B.2.1
Transmission of the T2-MIP over DVB-T2 ..................................................................................................... 42
B.2.2

Annex C (informative):

Local Content Insertion ................................................................................. 43

Annex D (informative):

MISO Management ....................................................................................... 44

Annex E (informative):

T2-MI overhead ............................................................................................. 45
Introduction ............................................................................................................................................ 45
E.0
E.1  Encapsulation of T2 data within T2-MI packets .................................................................................... 45
E.2  Transport of T2-MI packets ................................................................................................................... 45
T2-MI packets over MPEG-2 TS ..................................................................................................................... 45
E.2.1
Introduction................................................................................................................................................. 45
E.2.1.0
FEC overhead for an ASI link .................................................................................................................... 45
E.2.1.1
T2-MI packets over MPEG-2 TS to IP ............................................................................................................. 46
E.2.2
Introduction................................................................................................................................................. 46
E.2.2.0
FEC overhead ............................................................................................................................................. 46
E.2.2.1
E.3  Summary of the overheads associated with T2-MI ................................................................................ 46

DVB-T2 Timestamps ..................................................................................... 47
Annex F (informative):
F.1  Relationships .......................................................................................................................................... 47
F.2  Rationale ................................................................................................................................................. 47

Annex G (informative):

Use of T2-MI in Test and Measurement Setups .......................................... 48
Introduction ............................................................................................................................................ 48
G.1
G.2  Use of Program Clock Reference (PCR) timestamps ............................................................................. 48
Introduction ...................................................................................................................................................... 48
G.2.0
Relation between ISCR and PCR ..................................................................................................................... 48
G.2.1
Insertion of PCRs ............................................................................................................................................. 49
G.2.2
Playout of a Constant Bit-rate (CBR) T2-MI file ............................................................................................. 49
G.2.3
Playout of a Variable Bit-rate (VBR) T2-MI file ............................................................................................. 49
G.2.4
Synchronization between T2-Gateway and Modulator .................................................................................... 50
G.2.5

Annex H (normative):

T2-MI for Composite Signals ........................................................................ 51
Introduction ............................................................................................................................................ 51
H.1
H.2  Multiple T2-MI Streams ......................................................................................................................... 51
H.3  Alignment of the profiles in the emitted composite signal ..................................................................... 51

Annex I (informative):

T2-MI for Composite Signals: Network Topology and
Synchronization ............................................................................................. 53

ETSI

5

ETSI TS 102 773 V1.4.1 (2016-03)

Introduction ............................................................................................................................................ 53
I.1
I.2  Network Topology ................................................................................................................................. 53
Synchronization of Multiple T2-Gateways ............................................................................................ 54
Introduction ...................................................................................................................................................... 54
Configuration Changes and Multiple T2-Gateways ......................................................................................... 55

I.3
I.3.0
I.3.1

Change History .............................................................................................. 56
Annex J (informative):
History .............................................................................................................................................................. 57

ETSI

6

ETSI TS 102 773 V1.4.1 (2016-03)

Intellectual Property Rights

IPRs essential or potentially essential to the present document may have been declared to ETSI. The information
pertaining to these essential IPRs, if any, is publicly available for ETSI members and non-members, and can be found
in ETSI SR 000 314: "Intellectual Property Rights (IPRs); Essential, or potentially Essential, IPRs notified to ETSI in
respect of ETSI standards", which is available from the ETSI Secretariat. Latest updates are available on the ETSI Web
server (https://ipr.etsi.org/).

Pursuant to the ETSI IPR Policy, no investigation, including IPR searches, has been carried out by ETSI. No guarantee
can be given as to the existence of other IPRs not referenced in ETSI SR 000 314 (or the updates on the ETSI Web
server) which are, or may be, or may become, essential to the present document.

Foreword

This Technical Specification (TS) has been produced by Joint Technical Committee (JTC) Broadcast of the European
Broadcasting Union (EBU), Comité Européen de Normalisation ELECtrotechnique (CENELEC) and the European
Telecommunications Standards Institute (ETSI).

NOTE:  The EBU/ETSI JTC Broadcast was established in 1990 to co-ordinate the drafting of standards in the

specific field of broadcasting and related fields. Since 1995 the JTC Broadcast became a tripartite body
by including in the Memorandum of Understanding also CENELEC, which is responsible for the
standardization of radio and television receivers. The EBU is a professional association of broadcasting
organizations whose work includes the co-ordination of its members' activities in the technical, legal,
programme-making and programme-exchange domains. The EBU has active members in about
60 countries in the European broadcasting area; its headquarters is in Geneva.

European Broadcasting Union
CH-1218 GRAND SACONNEX (Geneva)
Switzerland
Tel:
+41 22 717 21 11
Fax:  +41 22 717 24 81

The Digital Video Broadcasting Project (DVB) is an industry-led consortium of broadcasters, manufacturers, network
operators, software developers, regulatory bodies, content owners and others committed to designing global standards
for the delivery of digital television and data services. DVB fosters market driven solutions that meet the needs and
economic circumstances of broadcast industry stakeholders and consumers. DVB standards cover all aspects of digital
television from transmission through interfacing, conditional access and interactivity for digital video, audio and data.
The consortium came together in 1993 to provide global standardisation, interoperability and future proof
specifications.

Modal verbs terminology

In the present document "shall", "shall not", "should", "should not", "may", "need not", "will", "will not", "can" and
"cannot" are to be interpreted as described in clause 3.2 of the ETSI Drafting Rules (Verbal forms for the expression of
provisions).

"must" and "must not" are NOT allowed in ETSI deliverables except when used in direct citation.

ETSI

7

ETSI TS 102 773 V1.4.1 (2016-03)

1

Scope

The present document defines the interface to a modulator for a second generation terrestrial television system
(DVB-T2). The present document also describes a mechanism to allow the operation of over the air regenerative
repeaters in SFN or non-SFN networks.

2

References

2.1

Normative references

References are either specific (identified by date of publication and/or edition number or version number) or
non-specific. For specific references, only the cited version applies. For non-specific references, the latest version of the
reference document (including any amendments) applies.

Referenced documents which are not found to be publicly available in the expected location might be found at
https://docbox.etsi.org/Reference/.

NOTE:  While any hyperlinks included in this clause were valid at the time of publication, ETSI cannot guarantee

their long term validity.

The following referenced documents are necessary for the application of the present document.

[1]

[2]

[3]

[4]

[5]

[6]

[7]

[8]

[9]

ETSI EN 302 755: "Digital Video Broadcasting (DVB); Frame structure channel coding and
modulation for a second generation digital terrestrial television broadcasting system (DVB-T2)".

ETSI TS 102 606: "Digital Video Broadcasting (DVB); Generic Stream Encapsulation (GSE)
Protocol".

ETSI TS 101 191: "Digital Video Broadcasting (DVB); DVB mega-frame for Single Frequency
Network (SFN) synchronization".

ETSI EN 301 192: "Digital Video Broadcasting (DVB); DVB specification for data broadcasting".

ETSI TS 102 034: "Digital Video Broadcasting (DVB); Transport of MPEG-2 TS Based DVB
Services over IP Based Networks".

IETF RFC 3550: "RTP: A Transport Protocol for Real-Time Applications".

ISO/IEC 13818-1: "Information technology - Generic coding of moving pictures and associated
audio information: Systems".

ETSI EN 300 468: "Digital Video Broadcasting (DVB); Specification for Service Information (SI)
in DVB systems".

ETSI TS 102 992: "Digital Video Broadcasting (DVB); Structure and modulation of optional
transmitter signatures (T2-TX-SIG) for use with the DVB-T2 second generation digital terrestrial
television broadcasting system".

2.2

Informative references

References are either specific (identified by date of publication and/or edition number or version number) or
non-specific. For specific references, only the cited version applies. For non-specific references, the latest version of the
reference document (including any amendments) applies.

NOTE:  While any hyperlinks included in this clause were valid at the time of publication, ETSI cannot guarantee

their long term validity.

The following referenced documents are not necessary for the application of the present document but they assist the
user with regard to a particular subject area.

[i.1]

ETSI TS 102 831: "Digital Video Broadcasting (DVB); Implementation guidelines for a second
generation digital terrestrial television broadcasting system (DVB-T2)".

ETSI

8

ETSI TS 102 773 V1.4.1 (2016-03)

[i.2]

[i.3]

[i.4]

CENELEC EN 50083-9: "Cable networks for television signals, sound signals and interactive
services - Part 9: Interfaces for CATV/SMATV headends and similar professional equipment for
DVB/MPEG-2 transport streams".

DVB BlueBook A115: "DVB Application Layer FEC Evaluations".

ETSI TR 101 290: "Digital Video Broadcasting (DVB); Measurement guidelines for DVB
systems".

3

Definitions, symbols and abbreviations

3.1

Definitions

For the purposes of the present document, the terms and definitions given in ETSI TS 102 831 [i.1] and the following
apply:

auxiliary stream: sequence of cells carrying data of as yet undefined modulation and coding, which may be used for
future extensions or as required by broadcasters or network operators

common PLP: PLP having one slice per T2 frame, transmitted just after the L1 signalling, which may contain data
shared by multiple PLPs

composite signal: signal made up of multiple T2 profiles, for example a combined T2-Base and T2-Lite transmission
with each signal being carried in the FEF part of the other

configurable L1-signalling: L1 signalling consisting of parameters which remain the same for the duration of one
super-frame

Coordinated Universal Time (literally Universel Temps Coordonné) (UTC): time format counting in standard SI
seconds with periodic adjustments made by the addition (or removal) of leap seconds to keep the difference between
UTC and Astronomical Time less than ±0,9 s

data PLP: PLP of Type 1 or Type 2

dynamic L1-signalling: L1 signalling consisting of parameters which may change from one T2-frame to the next

elementary period: time period which depends on the system bandwidth and is used to define the other time periods in
the T2 system

FEC Block: set of Ncells OFDM cells carrying all the bits of one LDPC FECFRAME

FECFRAME: set of Nldpc (16 200 or 64 800) bits from one LDPC encoding operation

FEF part: part of the super-frame between two T2-frames which may contain (FEFs)

FFT size: nominal FFT size used for a particular mode, equal to the active symbol period TS expressed in cycles of the
elementary period T

Global Position System (GPS): constellation of satellites providing accurate time and position information to receivers

GPS Time: time signal broadcast by the GPS satellites using an epoch of January 6th 1980 with no leap seconds and a
"week number" (actually a modulo-604 800 seconds number) that wraps every 1 024 weeks (approximately 19,7 years)

Im(x): imaginary part of x

interleaving frame: unit over which dynamic capacity allocation for a particular PLP is carried out, made up of an
integer, dynamically varying number of FEC blocks and having a fixed relationship to the T2-frames

NOTE:  The Interleaving frame may be mapped directly to one T2-frame or may be mapped to multiple

T2-frames. It may contain one or more TI-blocks.

International Atomic Time (literally Temps Atomique International) (TAI): time format counting in standard
SI seconds

ETSI

9

ETSI TS 102 773 V1.4.1 (2016-03)

L1 pre-signalling: signalling carried in the P2 symbols having a fixed size, coding and modulation, including basic
information about the T2 system as well as information needed to decode the L1 post-signalling

NOTE:  L1 pre-signalling remains the same for the duration of a super-frame.

L1-post-signalling: signalling carried in the P2 symbol carrying more detailed L1 information about the T2 system and
the PLPs

max: maximum of a set of numbers, the operator being defined as:

{
}
iX
)(

=

max

{
X

),1(

X

)...2(

})(

IX

max
=
i
I
..1

MISO group: group (1 or 2) to which a particular transmitter in a MISO network belongs, determining the type of
processing which is performed to the data cells and the pilots

NOTE:  Signals from transmitters in different groups will combine in an optimal manner at the receiver.

mod: modulo operator, defined as:

x mod

y

−=
x

y

⎢
⎢
⎣

⎥
⎥
⎦

x
y

Modified Julian Date (MJD): date format based on the number of days since midnight GMT on 17th
November 1858 AD

P1 signalling: signalling carried by the P1 symbol and used to identify the basic mode of the DVB-T2 signal

Physical Layer Pipe (PLP): physical layer TDM channel that is carried by the specified sub-slices

NOTE:  A PLP may carry one or multiple services.

PLP_ID: this 8-bit field uniquely identifies a PLP within the T2 system, identified with the T2_system_id

NOTE:  The same PLP_ID may occur in one or more frames of the super-frame.

relay (transmitter): transmitter in a network that re-transmits a signal received off-air, either by simple frequency
transposition or by regenerating the signal

Re(x): real part of x

Time Interleaving block (TI-block): set of cells within which time interleaving is carried out, corresponding to one
use of the time interleaver memory

Type 1 PLP: PLP having one slice per T2 frame, transmitted before any Type 2 PLPs

Type 2 PLP: PLP having two or more sub-slices per T2 frame, transmitted after any Type 1 PLPs

T2 frame: fixed physical layer TDM frame that is further divided into variable size sub-slices

NOTE:  The T2 frame starts with one P1 and one or multiple P2 symbols.

T2-Gateway: device producing T2-MI at its output, incorporating the functionality of the Basic T2-Gateway and,
optionally, additional processing such as re-multiplexing

T2-MI stream: stream of T2-MI packets carrying the T2 data for a single T2 profile and optionally any non-profile
data with a given value of T2-MI stream ID

T2-MI stream ID: unique identifier for a sequence of T2-MI packets presented to a modulator carrying a
self-consistent set of T2 data over T2-MI

T2 Super-frame: set of T2 frames consisting of a particular number of consecutive T2 frames

NOTE:  A super-frame may in addition include FEF parts.

ETSI

10

ETSI TS 102 773 V1.4.1 (2016-03)

T2 system: second generation terrestrial broadcast system whose input is one or more TS or GSE streams and whose
output is an RF signal

NOTE:  The T2 system:

(cid:0)

(cid:0)

(cid:0)

  Means an entity where one or more PLPs are carried, in a particular way, within a DVB-T2 signal

on one or more frequencies.

Is unique within the T2 network and it is identified with T2_system_id. Two T2 systems with the
same T2_system_id and network_id have identical physical layer structure and configuration,
except for the cell_id which may differ.

Is transparent to the data that it carries (including transport streams and services).

T2_SYSTEM_ID: this 16-bit field identifies uniquely the T2 system within the T2 network

3.2

Symbols

For the purposes of the present document, the symbols given in ETSI TS 102 831 [i.1] and the following apply:

Nx

The value N is expressed in radix x

NOTE:  The radix of x is decimal, thus 2A16 is the hexadecimal representation of the decimal number 42.

NT2
IFEF
IJUMP, IJUMP(i)

NPLP
PI , PI(i)
T
TF
TFEF
⎣ ⎦x
⎡ ⎤⎢ ⎥
x

Number of T2-frames in a super-frame
The value of FEF_INTERVAL from the L1-signalling
Frame interval: difference in frame index between successive T2 frames to which a particular PLP
is mapped (for PLP i)
Number of PLPs in a T2 system
Number of T2-frames to which each Interleaving Frame is mapped (for PLP i)
Elementary time period for the bandwidth in use
T2 Frame duration
Duration of one FEF part

Round towards minus infinity: the most positive integer less than or equal to x
Round towards plus infinity, i.e. the most negative integer greater than or equal to x

3.3

Abbreviations

For the purposes of the present document, the abbreviations given in ETSI TS 102 831 [i.1] and the following apply:

ACE
AL
AL-FEC
ASI
BB
bflbf
bflbfzpb
CBR
CRC
DFL
DVB
DVB-T
FEC
FEF
FFT
GMT
GPS
GSE
ID
IERS
IFFT
IP

Active Constellation Extension
Application Layer
Application Layer Forward Error Correction
Asynchronous Serial Interface
BaseBand
bit-field, left bit first
bit-field, left bit first, zero padded after the last bit to a multiple of 8 bits
Constant Bit-rate
Cyclic Redundancy Check
DataField Length
Digital Video Broadcasting
DVB system for Terrestrial Broadcasting
Forward Error Correction
Future Extension Frame
Fast Fourier Transform
Greenwich Mean Time
Global Positioning System
Generic Stream Encapsulation
IDentifier
International Earth Rotation and Reference Systems Service
Inverse Fast Fourier Transform
Internet Protocol

ETSI

11

ETSI TS 102 773 V1.4.1 (2016-03)

IPTV
IQ
ISCR
LDPC
LoCI
LSB
MFN
MISO
MJD
MPEG
MSB
MTU
OFDM
PAPR
PAT
PCR
PID
PLP
PMT
PRBS
PSI

Internet Protocol TeleVision
Inphase and Quadrature
Input Stream Time Reference
Low Density Parity Check (codes)
Local Content Inserter
Least Significant Bit
Multi-Frequency Network
Multiple Input, Single Output
Modified Julian Date
Moving Picture Experts Group
Most Significant Bit
Maximum Transmission Unit
Orthogonal Frequency Division Multiplex
Peak-to-Average Power Ratio
Program Association Table
Program Clock Reference
Packet Identifier
Physical Layer Pipe
Program Map Table
Pseudo Random Binary Sequence
MPEG-2 Program Specific Information

NOTE:  As defined in ISO/IEC 13818-1 [7].

RF
RFU
rms
rpchof
RTCP
RTP
SFN
SI
SMPTE
T2
T2-MI
T2-MIP
TAI

Radio Frequency
Reserved for Future Use
root mean square
remainder polynomial coefficients, highest order first
Real-Time Transport Control Protocol
Real Time Protocol
Single Frequency Network
Service Information
Society of Motion Picture and Television Engineers
DVB-T2
DVB-T2 Modulator Interface
DVB-T2 Modulator Information Packet
International Atomic Time

NOTE:  Literally Temps Atomique International.

TDM
TFS
TI
TPH
TR
TS
TX-SIG
UDP
uimsbf
UTC

Time Division Multiplex
Time Frequency Slicing
Time Interleaving
Transport Packet Header
Tone Reservation
Transport Stream
Transmitter Signature
User Datagram Protocol
unsigned integer, most significant bit first
Coordinated Universal Time

NOTE:  Literally Universel Temps Coordonné.

VBR
XOR

Variable Bit-rate
eXclusive OR function

ETSI

12

ETSI TS 102 773 V1.4.1 (2016-03)

4

General description

4.1

System overview

The DVB-T2 specification [1] enables service-specific robustness to be achieved through the use of Physical Layer
Pipes (PLPs). The allocation of data to each PLP is not however prescriptive and the T2 specification merely states that
certain constraints shall be met.

To enable Single Frequency Network (SFN) operation, decisions on allocation and scheduling are taken once in a
T2-Gateway, the results of which are distributed in such a format that each modulator in the network can
unambiguously create an identical on-air signal.

The T2-Gateway takes one or more input streams for the T2 system and forms them into un-coded Baseband frames and
generates the appropriate L1 signalling information to be sent over the T2-MI. The T2-Modulator uses this data from
the T2-MI and performs the necessary error coding, frame building and modulation to produce the RF signal for the T2
system.

The DVB-T2 Modulator Interface (T2-MI) allows reliable networks of transmitters (in both MFN and SFN
configurations) to be constructed. In addition it supports the use of regenerative, off-air repeaters to feed further MFNs
and SFNs.

More information regarding the generation of T2-MI in a T2-gateway and its use by a modulator can be found in ETSI
TS 102 831 [i.1].

The present document (version 1.3.1 of this specification) introduces the concept of multiple T2-MI streams to allow
the formation of composite T2 transmissions made up of multiple T2 profiles in accordance with clause I.9 of [1]. The
T2-Lite profile signal is distributed on one stream of T2-MI and transmitted in the FEF-part of a T2-Base signal which
is distributed on a second T2-MI stream. Similarly, The T2-Base profile signal is transmitted in the FEF part of a
T2-Lite signal. This is described in detail in annex H.

4.2

System architecture

The block diagram of a typical DVB-T2 end-to-end chain for Transport Stream input is shown in figure 1. The T2-MI is
shown as "Interface B" at the output of the T2-Gateway.

Figure 1: Block diagram of a typical DVB-T2 chain

ETSI

13

ETSI TS 102 773 V1.4.1 (2016-03)

4.3

Protocol stack

Figure 2 shows the T2-MI protocol stack.

Figure 2: The T2-MI protocol stack

The DVB-T2 Modulator Interface (T2-MI) carries the DVB-T2 system inputs, MPEG-2 TS and/or Generic Streams,
encapsulated within DVB-T2 Baseband Frames [1].

In addition the T2-MI also carries other T2 data including, but not limited to:

•

•

•

•

L1 signalling data to enable the construction of T2 frames by the modulator;

IQ vector data for any auxiliary streams;

DVB-T2 timestamp (for synchronization); and

Future Extension Frame (FEF) data.

With the exception of the DVB-T2 timestamp, all this information is transmitted as part of the on-air DVB-T2 signal.

The synchronization timestamp data is not transmitted over-air but used by a modulator to define the precise time of
emission of the DVB-T2 signal. A special case exists where relay stations forming part of a SFN are fed over air from a
master station on a different frequency, since they also require access to the synchronization data (see annex B).

The T2 data is packetized into T2-MI packets and encapsulated into DVB/MPEG Transport Stream packets using Data
Piping, in accordance with ETSI EN 301 192 [4], clause 4.

These standard DVB TS packets are then carried either natively over any standard DVB Transport Stream interface,
such as ASI [i.2], or further encapsulated within IP packets in accordance with ETSI TS 102 034 [5] for carriage over IP
based networks.

5

T2-MI packets

5.0

Introduction

Several different types of T2-related data may be sent over the T2-MI through the use of T2-MI packets. All fields are
uimsbf unless otherwise stated.

ETSI

14

ETSI TS 102 773 V1.4.1 (2016-03)

5.1

T2-MI packet definition

The T2-MI packet format is shown in figure 3.

Figure 3: T2-MI packet format

Each T2-MI packet is composed of a 6 byte header, followed by a variable-length payload part plus padding, when
required, and a 32-bit CRC tail for error detection.

The T2-MI packet consists of the following fields:

packet_type (8 bits) indicates the type of the payload carried by the T2-MI packet. The currently defined values are
shown in table 1 and their associated formats defined in the following clauses. All other values are Reserved for Future
Use (RFU).

Table 1: T2-MI packet types

T2-MI packet_type
0016
0116
0216
1016
1116
1216
2016
2116
3016
3116
3216
3316
all other values

Description
Baseband Frame
Auxiliary stream I/Q data
Arbitrary cell insertion
L1-current
L1-future
P2 bias balancing cells
DVB-T2 timestamp
Individual addressing
FEF part: Null
FEF part: I/Q data
FEF part: composite
FEF sub-part
Reserved for future use

packet_count (8 bits) is incremented by one for each T2-MI packet sent, irrespective of payload. There shall be no
requirement for the first packet sent to have a specific count value. The counter shall wrap from FF16 to 0016.

superframe_idx (4 bits) shall be constant for all T2-MI packets that carry data pertaining to one T2 super-frame. It
should be incremented for each subsequent super-frame. No implementation shall require this field to have any
particular value.

rfu (9 bits) bits reserved for future use and shall all be set to 02.

t2mi_stream_id (3 bits) shall have the same value for all T2-MI packets belonging to a particular T2-MI stream and is
used when transmitting a composite signal, in accordance with annex I, or to indicate that only a single stream is used.
When only a single stream is used, it shall be set to 0002.The T2-MI stream ID shall be unique within the set of T2-MI
streams presented to a single modulator.

payload_len (16 bits) indicates the payload length in bits.

payload (payload_len bits) carries the T2-MI packet payload which will vary depending on the type of the T2-MI
packet and is defined in clause 5.2.

ETSI

15

ETSI TS 102 773 V1.4.1 (2016-03)

pad (pad_len bits) shall be filled with between 0 and 7 bits of padding such that the T2-MI packet is always an integer
number of bytes in length, i.e. payload_len+pad_len shall be a multiple of 8. Each padding bit shall have the value 02.

crc32 (32 bits) is calculated across all other bits in the packet (both header and payload plus any padding), in
accordance with annex A.

5.2

T2-MI payload definitions

5.2.1

Baseband Frame

T2-MI packets with a packet_type of 0016 shall carry Baseband Frames, in accordance with ETSI EN 302 755 [1],
clause 5.1.7.

The T2-MI packet payload is shown in figure 4.

frame_idx

plp_id

intl_frame
_start

rfu

8 bits

8 bits

1 bit

7 bits

BBFRAME

Kbch bits

Figure 4: Baseband Frame payload

The fields are defined as follows:

frame_idx (8 bits) indicates the FRAME_IDX, as defined in ETSI EN 302 755 [1], of the first T2 frame to which the
Interleaving Frame containing this Baseband Frame is mapped.

plp_id (8 bits) signals the PLP_ID, as defined in ETSI EN 302 755 [1], in which the Baseband Frame is to be carried in
the DVB-T2 signal.

intl_frame_start (1 bit) shall be set to 12 for the packet containing the first BBFRAME of an interleaving frame for a
particular PLP, and 02 for packets carrying the remaining BBFRAMEs (if any).

rfu (7 bits) bits reserved for future use and shall all be set to 02.

BBFRAME (Kbch bits) carries the Kbch bits of the Baseband Frame (before scrambling) pertaining to a particular PLP,
including the PADDING field if used. It shall be encapsulated into exactly one T2-MI packet without additional
stuffing. The temporal order of the Baseband Frame bits shall be preserved. If the Baseband Frame PADDING field is
used for in-band signalling, the relevant bits of the PADDING field shall be set to "0". These shall then be replaced by
the relevant in-band signalling in the modulator.

5.2.2

Auxiliary stream I/Q data

T2-MI packets with a packet_type of 0116 shall carry auxiliary stream data, in accordance with ETSI EN 302 755 [1],
clause 8.3.7.

The T2-MI packet payload is shown in figure 5.

frame_idx

aux_id

rfu

aux_stream_data

8 bits

4 bits

12 bits

variable

Figure 5: Auxiliary stream payload

frame_idx (8 bits) indicates the FRAME_IDX, as defined in ETSI EN 302 755 [1], of the T2 frame which carries the
auxiliary stream data.

aux_id (4 bits) indicates the particular auxiliary stream to which the data belongs. The auxiliary streams shall be sent in
the same order as over the transmitted DVB-T2 signal, starting with aux_id=116 indicating the first auxiliary stream and
with the aux_id being incremented by "1" for each new auxiliary stream. The highest possible value is F16
corresponding to the 15th auxiliary stream. Other values are reserved for future use.

ETSI

16

ETSI TS 102 773 V1.4.1 (2016-03)

rfu (12 bits) bits reserved for future use.

aux_stream_data (variable bits) carries the data for each auxiliary stream. It shall consist of the complex cell values in
order of increasing cell address (as defined in ETSI EN 302 755 [1]). Each cell value shall be sent as a 12-bit two's
complement value I for the real part immediately followed by a 12-bit two's complement value Q for the imaginary part
of the complex number. The cell value, xm,l,p for use in clause 8.3.7 of ETSI EN 302 755 [1] shall be given by:

Re(

x
,,
plm

)

Im(

x
,,
plm

)

=

=

I
9
2
Q
9
2

where I and Q are the 12-bit two's complement values represented as integers in the range -211 to 211-1.

NOTE 1:  Given that the rms value of xm,l,p is equal to 1 (as required by ETSI EN 302 755 [1]), the

signal-to-quantization-noise ratio is approximately 59 dB, which should be adequate for all applications.

The auxiliary stream data field shall be encapsulated into one or more T2-MI packets in the same order as the filling of
the OFDM cells in the DVB-T2 signal. No stuffing shall be used.

If more than one T2-MI packet is used for a particular auxiliary stream the payload of T2-MI packets with an unfinished
stream shall end with a completed cell. The next cell value of that stream will then start at the beginning of the payload
of the next T2-MI packet with packet_type 0116 with the same aux_id.

The cell values for auxiliary streams shall be the same for all transmitters in a single frequency network when sent over
the T2-MI using T2-MI packet_type 0116.

NOTE 2:  If it is required, however, that the cell values are to differ, as allowed by ETSI EN 302 755 [1], the
auxiliary stream data need to be sent to the modulators in an alternative way.

5.2.3

Arbitrary cell insertion

T2-MI packets with a packet_type of 0216 shall carry arbitrary cell data that the modulator shall insert into the T2
frame starting at the specified cell address and continuing until the end of the complex cell values in the
aribitrary_cell_data field.

The T2-MI packet payload is shown in figure 6.

frame_idx

tx_identifier

rfu

start_cell_address

arbitrary_cell_data

8 bits

16 bits

18 bits

22 bits

variable

Figure 6: Arbitrary cell insertion payload

frame_idx (8 bits) indicates the FRAME_IDX, as defined in ETSI EN 302 755 [1], of the T2 frame which carries the
arbitrary cell data.

tx_identifier (16 bits) is a word used to address individual transmitters or modulators. This field has the same meaning
as in clause 5.2.8. A value of 000016 is used as a broadcast address to address all transmitters or modulators in the
network. Data from previous packets can be overwritten by later packets received by a particular modulator.

rfu (18 bits) bits reserved for future use and shall all be set to 02 until defined.

start_cell_address (22 bits) indicates the start address of the arbitrary cell data using the cell addressing scheme
specified in clause 8.3.6.2 of ETSI EN 302 755 [1].

ETSI

17

ETSI TS 102 773 V1.4.1 (2016-03)

arbitrary_cell_data (variable bits) carries the arbitrary cell data to be inserted by the modulator. It shall consist of the
complex cell values in order of increasing cell address (as defined in ETSI EN 302 755 [1]). Each cell value shall be
sent as a 12-bit two's complement value I for the real part immediately followed by a 12-bit two's complement value Q
for the imaginary part of the complex number. The cell value, xm,l,p for use in clause 8.3.7 of ETSI EN 302 755 [1] shall
be given by:

Re(

x
,,
plm

)

Im(

x
,,
plm

)

=

=

I
9
2
Q
9
2

where I and Q are the 12-bit two's complement values represented as integers in the range -211 to 211-1.

NOTE:  Given that the rms value of xm,l,p is equal to 1 (as required by ETSI EN 302 755 [1]), the

signal-to-quantization-noise ratio is approximately 59 dB, which should be adequate for all applications.

If it is required to carry more arbitrary cell data than can be conveyed in a single T2-MI packet then the cell data shall
be split across multiple T2-MI packets of type 0216 with appropriate values of start_cell_address. Each T2-MI packet
shall end with a completed cell.

5.2.4

L1-current T2-MI packets

T2-MI packets with a packet_type of 1016 shall contain L1 pre- and post-signalling data to be inserted (as described in
clause 5.3) into the P2 symbols of the T2-frame indicated by frame_idx and describing the same ("current") T2-frame.

The T2-MI packet payload is shown in figure 7.

frame_idx

freq_
source

rfu

L1-current_data

8 bits

2 bits

6 bits

variable

Figure 7: L1-current data payload

frame_idx (8 bits) indicates the FRAME_IDX according to ETSI EN 302 755 [1] of the T2-frame in which the L1
signalling data is carried. This is also the T2-frame that the L1 signalling data describes.

freq_source (2 bits) indicates the source of the FREQUENCY field of the configurable L1-post signalling in the
transmitted DVB-T2 signal. The coding shall be as follows:

•

•

•

•

freq_source='00': the FREQUENCY field(s) of the DVB-T2 signal shall be according to the signalled value(s)
in the L1-current data field of the T2-MI signal without further modification at the modulator. When the
frequency individual addressing function is available in the T2-MI signal this shall be ignored.

freq_source='01': the FREQUENCY field(s) of the DVB-T2 signal shall be according to the T2-MI frequency
individual addressing function without further modification at the modulator. The FREQUENCY field(s) of
the L1-current data field of the T2-MI signal shall be ignored.

freq_source='10': the FREQUENCY field(s) of the DVB-T2 signal shall be according to the manually set
value(s) for each modulator. Both the FREQUENCY field(s) of the L1-current data field of the T2-MI signal
and any available frequency function shall be ignored.

freq_source='11': reserved for future use.

rfu (6 bits) bits reserved for future use and shall all be set to 02.

L1-current_data contains fields in the order given in table 2.

NOTE 1:  The P1 signalling is generated by the modulator from the S1 and S2 fields in the L1 pre-signalling (see

clause 7.2.2 of ETSI EN 302 755 [1]).

ETSI

18

ETSI TS 102 773 V1.4.1 (2016-03)

Table 2: L1-current data fields

Field

Field length (bits)

L1PRE

168

L1CONF_LEN
L1CONF

16
8

⎡
L×

_1

CONF

_

LEN

⎤8

Format
bflbf

uimsbf
bflbfzpb

Description
L1 pre-signalling bits in the order defined in
clause 7.2.2 of [1], excluding the CRC.
Length of L1 configurable signalling in bits
L1 configurable post-signalling fields, in the
order defined in clause 7.2.3.1 of [1].

L1DYN_CURR_LEN
L1DYN_CURR

16
8

⎡
L×
DYN
1

_

CURR

_

LEN

uimsbf
  bflbfzpb

⎤8

Length of L1-dynamic, current frame.
L1-post "dynamic, current frame" fields in
the order defined in clause 7.2.3.2 of [1].

L1EXT_LEN
L1EXT

16
8

⎡
L×

_1

EXT

_

LEN

⎤8

uimsbf
bflbfzpb

Length of L1 extension field, in bits.
L1-post extension field as defined in
clause 7.2.3.4 of [1].

The L1PRE, L1CONF and L1DYN_CURR fields are mandatory in all L1-current T2-MI packets and shall be coded in
accordance with clauses 7.2.2, 7.2.3.1 and 7.2.3.2 respectively of ETSI EN 302 755 [1] and without any L1-post
scrambling being applied.

NOTE 2:  The L1DYN_CURR field will not be transmitted in P2 in TFS mode, but is mandatory because the

information will be used by the modulator for Interleaving and Frame-Building.

NOTE 3: In the TFS mode each transmitted RF frequency uses a unique value of the CURRENT_RF_IDX field of

the L1-pre signalling. The T2-MI signal conveys however only a single value in the L1_current data field.
The correct value of CURRENT_RF_IDX for all the transmitted RF frequencies can however be inferred
from the FREQUENCY loop of the configurable L1 signalling, the frequency individual addressing
function or from the manually set parameters, as determined from the setting of the freq_source field.

5.2.5

L1-future

T2-MI packets with a packet_type of 1116 shall contain L1 post-signalling data to be inserted (according to clause 7.2.3
of [1]) into the P2 symbols of the T2-frame indicated by frame_idx, and/or in-band signalling data to be inserted into
the first BB-Frame of the Interleaving Frame beginning in that T2-frame. The signalling contained comprises those
fields that describe future T2-frames, and might therefore not be available at the time the L1-current T2-MI packet is
sent. The T2-MI packet payload is shown in figure 8.

frame_idx

rfu

8 bits

8 bits

L1-future_data

variable

Figure 8: L1-future data payload

frame_idx (8 bits) indicates the FRAME_IDX according to ETSI EN 302 755 [1] of the T2 frame in whose P2 symbols
the L1 dynamic post-signalling data is carried. It also indicates the first T2-frame carrying the Interleaving Frame
whose first BB-Frame will contain the in-band signalling.

Which T2-frame is described by the dynamic post-signalling and in-band signalling will depend on the use of TFS as
described in clauses 7 and 5.2.3 respectively of ETSI EN 302 755 [1]. Which T2-frame or frames are described by the
in-band signalling will also depend on the interleaving parameters (PI and Ijump) for the PLP in which they are inserted,
as described in clause 5.2.3 of ETSI EN 302 755 [1].

rfu (8 bits) bits reserved for future use and shall all be set to 02.

L1-future_data contains fields in the order given in table 3.

ETSI

19

ETSI TS 102 773 V1.4.1 (2016-03)

Table 3: L1-future data fields

Field

Field length (bits)

L1DYN_NEXT_LEN

16

L1DYN_NEXT

8

⎡
L×
DYN
1

_

NEXT

_

LEN

⎤
8

Format
uimsbf

Description
Length of "dynamic, next frame" field. Set to
zero if L1DYN_NEXT block is absent.
bflbfzpb  L1-post "dynamic, next frame" fields.

L1DYN_NEXT2_LEN

16

uimsbf

L1DYN_NEXT2

⎡
L×
DYN
1

8

_

NEXT

_2

LEN

⎤8

NUM_INBAND

For
i=1..NUM_INBAND {
  PLP_ID

8

8

INBAND_LEN
INBAND

16
×
8

⎡
INBAND

_

LEN

⎤
8

}

Optional in single-RF mode, mandatory in
TFS.
Length of "dynamic, next-but-one frame" in
TFS mode. Set to zero if L1DYN_NEXT2
block is absent.

bflbfzpb  L1-post "dynamic, next-but-one frame"

uimsbf

uimsbf

bflbfzpb

fields, in the order defined in clause 7.2.3.2
of ETSI EN 302 755 [1]. Optional in TFS,
and shall not be present in single-RF mode.
Number of PLPs for which in-band signalling
is present in the following loop.
In-band signalling loop.

PLP ID for the PLP containing the in-band
signalling data given by the following
INBAND field.
Length of following INBAND field in bits.
In-band signalling fields for the PLP
indicated by PLP_ID, in the order defined in
clause 5.2.3 of ETSI EN 302 755 [1].

Only PLPs for which the T2-frame indicated by frame_idx is the first T2-frame to which an Interleaving Frame is
mapped shall appear in the in-band signalling loop.

The L1DYN_NEXT and L1DYN_NEXT2 fields shall be coded in accordance with clause 7.2.3.2 of ETSI
EN 302 755 [1]. The INBAND fields shall be coded in accordance with clause 5.2.3 of ETSI EN 302 755 [1] and
without any L1-post scrambling being applied.

5.2.6

P2 bias balancing cells

T2-MI packets with a packet_type of 1216 shall contain information regarding bias balancing cells to be inserted by the
modulator (according to clause 8.3.6.3.1 of ETSI EN 302 755 [1]) into the P2 symbols of the T2-frame indicated by
frame_idx to approximately bias the balance of the L1 signalling. This packet instructs a modulator how many bias
balancing cells to insert in each P2 symbol. The calculation of the actual value of the bias balancing cells, Cbal shall be
performed by the modulator on the coded and modulated L1 cells.

The T2-MI packet payload is shown in figure 9.

frame_idx

rfu

num_active_bias_cells_per_p2

8 bits

17 bits

15 bits

Figure 9: P2 bias balancing cells payload

frame_idx (8 bits) indicates the FRAME_IDX, as defined in ETSI EN 302 755 [1], of the T2 frame which carry the
bias balancing cells.

rfu (17 bits) bits reserved for future use and shall all be set to 02.

num_active_bias_cells_per_p2 (15 bits) indicates the number of bias balancing cells to be used in every P2 symbol of
the T2 frame as follows:

N

biasCellsA

ctive

=

num

_

active

_

bias

_

cells

_

per

p
2_

ETSI

20

ETSI TS 102 773 V1.4.1 (2016-03)

5.2.7

DVB-T2 timestamp

5.2.7.0

Introduction

T2-MI packets with a packet_type of 2016 shall carry the DVB-T2 timestamp, used to synchronize the output of
DVB-T2 modulators. Two mechanisms are defined; absolute and relative.

The T2-MI packet payload for this data is shown in figure 10.

rfu

bw

T2_timestamp

seconds_since_2000

4 bits

4 bits

40 bits

subseconds

27 bits

utco

13 bits

rfu (4 bits) bits reserved for future use and shall all be set to 02.

Figure 10: DVB-T2 timestamp payload

bw (4 bits) indicates the system bandwidth, in accordance with clause 9.5 of ETSI EN 302 755 [1]. This also defines
the units of the subsecond field of the T2 timestamp as shown in table 4.

Table 4: Bandwidths and subsecond field units for the T2 timestamp

Bandwidth

bw field

T2 Elementary
period, T

1,7 MHz
5 MHz
6 MHz
7 MHz
8 MHz
10 MHz

016
116
216
316
416
516

71/131 µs
7/40 µs
7/48 µs
7/56 µs
7/64 µs
7/80 µs

subseconds
unit, Tsub
1/131 µs
1/40 µs
1/48 µs
1/56 µs
1/64 µs
1/80 µs

seconds_since_2000 (40 bits) is a count of the number of seconds since 2000-01-01 T 00:00:00 UTC as an unsigned 40-
bit quantity and is used to define an absolute time of emission. This count shall increase for every SI second that elapses.
A value of 000000000016 indicates a relative timestamp, defined only by the subseconds field below.

subseconds (27 bits) defines the number of subsecond units since the time expressed in the seconds field. The value is
expressed as an unsigned integer.

T2_timestamp: Taken together, the seconds_since_2000 and subseconds fields define the DVB-T2 timestamp and the
time of emission of a DVB-T2 transmission. Annex F details the relationship between the DVB-T2 timestamp and other
time standards.

When the seconds_since_2000 field is non-zero, the emission time shall be given by
seconds_since_2000 + subseconds × Tsub.

When the seconds_since_2000 field is all zeros, the emission time shall be subseconds × Tsub after the SI second
boundary preceding it.

NOTE 1:  The SI second boundary can be given by the relevant edge of a 1 pulse per second signal.

The emission time shall be the time at which 50 % of the energy of the first time sample from the IFFT of the "C" part
of the P1 preamble symbol of the first T2 transmission frame of the relevant super-frame shall have been radiated on
air. All T2 frames within a super-frame shall have the same timestamp value. The timestamps of subsequent
super-frames shall be increased by the duration of the super-frame.

NOTE 2:  Based on the knowledge of the DVB-T2 Timestamp of a particular super-frame, and the L1 signalling
pertaining to a particular T2 frame, a modulator should be able to determine the required emission time
for any such T2 frame even if it misses the beginning of a super-frame, e.g. after a restart. To do this, the
modulator will then need to take into account the frame index and the frame length of the T2 frame as
well as the total lengths of any FEF parts having occurred in the super-frame before the current T2 frame.

ETSI

21

ETSI TS 102 773 V1.4.1 (2016-03)

utco (13 bits) is the offset (in seconds) between UTC and the seconds_since_2000 field. The value is expressed as an
unsigned integer. As of February 2009, the value shall be 2 and shall change as a result of each new leap second
proscribed by the International Earth Rotation and Reference Systems Service (IERS).

NOTE 3:  The value contained in this field has no effect on the time of emission from the modulator but it may be

useful to a modulator implementation where only a source of UTC time is available.

NOTE 4:  The maximum latency of the distribution system, plus the maximum processing delay of modulator

implementations (i.e. the relevant Tmin values from clause 5.5) for the mode being broadcast will need to
be known. Where the range of total delays exceeds 1 second the use of the Absolute T2 Timestamp will
be necessary to avoid ambiguous super-frame start times.

5.2.7.1

Null timestamp

When it is not required to synchronize the output of multiple DVB-T2 modulators, the DVB-T2 timestamp shall be
signalled as null by setting all bits of the T2_timestamp and utco fields to 12. When generating a composite signal,
even in an MFN, Null timestamps shall not be used, to ensure the correct relative timing of the different T2-MI streams.

A DVB-T2 timestamp packet shall always be sent (whether carrying a Null timestamp or otherwise) to indicate the
bandwidth of the T2 transmission to the T2 modulator.

5.2.8

Individual addressing

5.2.8.0

Introduction

T2-MI packets with a packet_type of 2116 shall carry individual addressing data that can be used to configure an
individual or group of modulators. In the TFS case a 'modulator' may either refer collectively to the modulation
equipment of the entire multi-frequency TFS signal, or to the modulation equipment of one of the RF frequencies of the
TFS signal.

The individual addressing mechanism is asynchronous and packets can be sent at any time. It is used by the modulator
to update register values as and when these packets are received. The modulator shall use the L1 signalling as its
primary source of information on how to construct the overall DVB-T2 frame, making reference to register values when
required.

The T2-MI packet payload is shown in figure 11 and the individual addressing data is in the same format as that
described in clause 6.1 of ETSI TS 101 191 [3].

Figure 11: Individual addressing payload

individual_addressing_length (8 bits) indicates the length of the individual_addressing_data field in byes.

individual_addressing_data (variable bits) is composed as follows:

•

•

tx_identifier (16 bits) is a word used to address individual transmitters or modulators. A value of 000016 is
used as a broadcast address to address all transmitters or modulators in the network.

function_loop_length (8 bits) indicates the length of the following loop of functions in bytes.

ETSI

22

ETSI TS 102 773 V1.4.1 (2016-03)

function() is the addressing function and is dependent on the application. They are defined in clauses 5.2.6.1
and 5.2.6.2.

5.2.8.1

Existing addressing functions

The format of the individual addressing functions is in accordance with clause 6.1 of ETSI TS 101 191 [3]. Table 5
indicates which of the currently defined functions are also applicable to DVB-T2.

Table 5: Existing individual addressing functions

Function
Transmitter time offset
Transmitter frequency offset
Transmitter power
Private data
Cell id
Enable
Bandwidth

function_tag value  Applicable to DVB-T2?

0016
0116
0216
0316
0416
0516
0616

yes
yes
yes
yes
yes
yes
no

5.2.8.2

Addressing functions specific to DVB-T2

5.2.8.2.0

Introduction

Some new functions are defined to fully support DVB-T2 as depicted in table 6. Whilst these have the same basic
structure as those defined in clause 6.1 of ETSI TS 101 191 [3], the data they carry is specific to their function in a T2
system.

Table 6: Individual addressing functions specific to DVB-T2

Function
ACE-PAPR
Transmitter MISO group
TR-PAPR
L1-ACE-PAPR
TX-SIG FEF: Sequence Numbers
TX-SIG Aux stream: Transmitter ID
Frequency

function_tag value
1016
1116
1216
1316
1516
1616
1716

Each function() is constructed from three fields as follows:

function_tag (8 bits) is the value identifying the particular function in use as defined in tables 5 and 6.

function_length (8 bits) defines the total length of the function() in bytes, including the function_tag,
function_length and function_body() fields.

function_body() is specific to the particular individual addressing function as defined in the clauses below.

NOTE:  For each of the existing individual addressing functions defined in ETSI TS 101 191 [3],
function_body() comprises all the fields that follow the function_length field.

5.2.8.2.1

ACE-PAPR function

The ACE-PAPR function is used to signal the Active Constellation Extension (ACE) parameters to the DVB-T2
modulator (see clause 9.6.1 of ETSI EN 302 755 [1]). ACE has 3 parameters, G, L and Vclip that can be used by all the
modulators that are part of an SFN to ensure that they produce identical on-air signals. Table 7 shows the format of the
individual addressing function.

ETSI

23

ETSI TS 102 773 V1.4.1 (2016-03)

Table 7: ACE-PAPR function

Syntax

Number of bits

Format

tx_ACE_PAPR_function() {

function_tag
function_length
function_body() {
  ACE_gain
  ACE_maximal_extension
  ACE_clipping_threshold
reserved_for_future_use

}

}

8
8

5
3
7
1

uimsbf
uimsbf

uimsbf
uimsbf
uimsbf
bflbf

ACE_gain (5 bits) shall be a value between 0 and 31 that represents the ACE gain, G.

ACE_maximal_extension (3 bits) shall be a value that represents the ACE maximal extension value, L as follows:

=

L

ACE_maxima

l_extensio
n
10

7+

ACE_clipping_threshold (7 bits) shall be a value that represents the ACE clipping threshold, Vclip as follows:

V

=
clip V

rms

ACE

_

clipping

_

threshold

200

10.

A value for ACE_clipping_threshold of 11111112 shall indicate that Vclip = +

∞

reserved_for_future_use (1 bit) is reserved for future use and shall be set to 02 until defined.

5.2.8.2.2

MISO group function

This function allows the MISO group (see clause 9.6.1 of ETSI EN 302 755 [1] and annex D) to be signalled to a DVB-
T2 modulator. Table 8 shows the format of the individual addressing function.

Table 8: MISO group function

Syntax
tx_MISO_function() {
function_tag
function_length
function_body() {
  MISO_group

reserved_for_future_use

}

}

Number of bits

Format

8
8

1
7

uimsbf
uimsbf

bflbf
bflbf

MISO_group (1 bit) indicates the MISO group. Value 02 indicates MISO group 1. Value 12 indicates MISO group 2.

reserved_for_future_use (7 bits) is reserved for future use and shall all be set to 02 until defined.

ETSI

24

ETSI TS 102 773 V1.4.1 (2016-03)

5.2.8.2.3

TR-PAPR function

The TR-PAPR function is used to signal the Tone Reservation (TR) parameters to the DVB-T2 modulator (see
clause 9.6.2 of ETSI EN 302 755 [1]). TR has a single parameter Vclip that can be used by all the modulators that are
part of an SFN to ensure that they produce identical on-air signals. Table 9 shows how these parameters are conveyed in
the addressing function.

Table 9: TR-PAPR function

Syntax

Number of bits

Format

tx_TR_PAPR_function() {

function_tag
function_length
function_body() {

reserved_for_future_use1
TR_clipping_threshold
reserved_for_future_use2
number_of_iterarions

}

}

8
8

4
12
14
10

uimsbf
uimsbf

bflbf
uimsbf

bflbf

reserved_for_future_use1 (4 bits) are reserved for future use and shall all be set to 02 until defined.

TR_clipping_threshold (7 bits) shall be a value that represents the TR clipping threshold, Vclip (measured in volts) as
follows:

=
clip V
V

rms

10.

TR

_

clipping

_

threshold

2

000

A value for TR_clipping_threshold of FFF16 shall indicate that Vclip = +

∞

, i.e. that the TR-PAPR has no effect.

NOTE:  The derivation of Vclip from this field differs from that used for the ACE-PAPR function defined in

clause 5.2.8.2.1.

reserved_for_future_use2 (14 bits) is reserved for future use and shall be all set to 02 until defined.

number of iterations (10 bits) is the number of iterations of the TR algorithm to use. A value of 11111111112
indicates that the modulator may use as many or as few as deemed necessary.

If the T2_VERSION field in the L1 signalling is set to a value greater than 00002 and the PAPR field is set to a value of
00002 then this function shall signal the Vclip value for the single iteration of tone reservation applied to the P2 symbols
only (see clause 9.6.2 of ETSI EN 302 755 [1]). In this case, the number_of_iterations shall be set to 00000000012.
All other values are reserved for future use.

5.2.8.2.4

L1-ACE-PAPR function

The L1-ACE-PAPR function is used to signal the Active Constellation Extension (ACE) parameters that are applied to
the use of ACE on the L1 signalling cells (only) to the DVB-T2 modulator (see clause 9.6.1 of ETSI EN 302 755 [1]).
The L1-ACE has one parameter CL1_ACE_MAX that can be used by all the modulators that are part of an SFN to ensure
that they produce identical on-air signals. Table 10 shows the format of the individual addressing function.

ETSI

25

ETSI TS 102 773 V1.4.1 (2016-03)

Table 10: L1-ACE-PAPR function

Syntax

Number of bits

Format

tx_L1_ACE_PAPR_function() {

function_tag
function_length
function_body() {

L1_ACE_max_correction
reserved_for_future_use

}

}

8
8

16
16

uimsbf
uimsbf

uimsbf
bflbf

L1_ACE_max_correction (16 bits) shall be a value that represents the maximum L1-ACE correction, CL1_ACE_MAX as
follows:

C

L
_1

ACE

_

MAX

=

L1_ACE_max

_correctio
n

1

000

A value of L1_ACE_max_correction of 000016 means that no correction will be made by the L1-ACE algorithm.

reserved_for_future_use (16 bits) is reserved for future use and shall be set to 000016 until defined.

5.2.8.2.5

TX-SIG FEF Sequence Numbers function

This function is used to signal the sequence numbers used by a DVB-T2 modulator when generating a Transmitter
Signature contained within a FEF part (see clause 6 of ETSI TS 102 992 [9]). The Transmitter Signature has two values
of a parameter h that are used to select which Generalised Orthogonal Sequence Sh is transmitted in each of the first and
second signature periods. Table 11 shows the format of the individual addressing function.

Table 11: TX-SIG FEF Sequence Numbers function

Syntax
tx_TX_SIG_SEQ_NUM_function() {

Number of bits

Format

function_tag
function_length
function_body() {

reserved_for_future_use1
TX_SIG_FEF_SEQ_NUM_1
reserved_for_future_use2
TX_SIG_FEF_SEQ_NUM_2
reserved_for_future_use3

8
8

5
3
5
3
24

uimsbf
uimsbf

bflbf
uimsbf
bflbf
uimsbf
bflbf

}

}

reserved_for_future_use1 (5 bits) is reserved for future use and all bits shall be set to 02 until defined.

TX_SIG_FEF_SEQ_NUM_1 (3 bits) shall be the value h indicating which Generalised Orthogonal (GO) sequence Sh
is to be transmitted in the first signature period.

reserved_for_future_use2 (5 bits) is reserved for future use and all bits shall be set to 02 until defined.

TX_SIG_FEF_SEQ_NUM_2 (3 bits) shall be the value h indicating which Generalised Orthogonal (GO) sequence Sh
is to be transmitted in the second signature period.

reserved_for_future_use3 (24 bits) is reserved for future use and all bits shall be set to 02 until defined.

ETSI

26

ETSI TS 102 773 V1.4.1 (2016-03)

5.2.8.2.6

TX-SIG aux stream transmitter ID function

This function is used to signal the transmitter ID used by a DVB-T2 modulator when generating a Transmitter Signature
contained within an auxiliary stream (see clause 5 of ETSI TS 102 992 [9]). The Transmitter Signature denotes the
individual transmitters to be signalled as having a transmitter ID of tx_id_1, tx_id_2, … tx_id_M, i.e. tx_id_m where
m=1..M. The function signals the value of m for a given transmitter. Table 12a shows the format of the individual
addressing function.

NOTE:  This value tx_id_M above that denotes a transmitter signature parameter should not be confused with

tx_identifier which is used to address individual modulators or transmitters fed by a T2-MI stream.

Table 12a: TX-SIG aux stream transmitter ID function

Syntax

Number of bits

Format

tx_TX_SIG_ AUX_TX_ID_function() {

function_tag
function_length
function_body() {

TX_SIG_AUX_TX_ID
reserved_for_future_use

}

}

8
8

12
20

uimsbf
uimsbf

uimsbf
bflbf

TX_SIG_AUX_TX_ID (12 bits) shall be the value that represents the transmitter identifier m as follows:
TX_SIG_AUX_TX_ID = m. The value of 00016 shall be reserved for future use.

reserved_for_future_use (20 bits) is reserved for future use and all bits shall all set to 02 until defined.

NOTE:  The other relevant transmitter signature parameters are carried in the L1 signalling.

5.2.8.2.7

Frequency function

This function is used to signal the FREQUENCY field(s) of the configurable L1-post signalling to be produced by a
DVB-T2 modulator. Table 12b shows the format of the Frequency function.

Table 12b: Frequency function

Syntax

Number of bits

Format

frequency_function() {

function_tag
function_length
function_body() {

rf_idx
frequency
reserved

}

}

8
8

3
32
5

uimsbf
uimsbf

uimsbf
uimsbf
bflbf

NOTE:

In the TFS case the Frequency function supports both the single and multiple tx_identifier variants for
transmitter addressing, as specified in Claue 5.2.8.0. In the latter case the frequency loop of the Frequency
function includes a single frequency value.

5.2.9

FEF part: Null

T2-MI packets with a packet_type of 3016 shall carry information related to a FEF part, in accordance with ETSI
EN 302 755 [1], clause 8.4, during which no signal shall be generated by a single profile modulator apart from the P1
preamble.

The T2-MI packet payload is shown in figure 12.

ETSI

27

ETSI TS 102 773 V1.4.1 (2016-03)

fef_idx

rfu

s1_field

s2_field

8 bits

9 bits

3 bits

4 bits

Figure 12: FEF part: Null payload

fef_idx (8 bits) indicates the index of the FEF part within the super-frame. The first FEF part in a super-frame shall
have a fef_idx value of 0 and this shall increment by 1 for each subsequent FEF part.

rfu (9 bits) bits reserved for future use and shall all be set to 02 until defined.

s1_field gives the value of the S1 field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

s2_field gives the value of the S2 field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

Unless the content for the corresponding FEF part is specified by another means, the modulator shall generate a P1
preamble according to the s1_field and s2_field followed by zero modulation values for the remainder of the FEF part.

5.2.10  FEF part: I/Q data

T2-MI packets with a packet_type of 3116 shall carry information related to a FEF part, in accordance with ETSI
EN 302 755 [1], clause 8.4, together with I/Q data to be transmitted during the FEF part.

The T2-MI packet payload is shown in figure 13.

fef_idx

rfu

s1_field

s2_field

fef_part_data

8 bits

9 bits

3 bits

4 bits

variable

Figure 13: FEF part: I/Q data payload

fef_idx (8 bits) indicates the index of the FEF part within the super-frame. The first FEF part in a super-frame shall
have a fef_idx value of 0 and this shall increment by 1 for each subsequent FEF part.

rfu (9 bits) bits reserved for future use and shall all be set to 02 until defined.

s1_field gives the value of the "S1" field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

s2_field gives the value of the "S2" field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

fef_part_data carries the IQ data for each FEF part. It shall consist of the complex sample values in time order, starting
from the first sample after the end of the P1 preamble, at a sampling rate of 1/T as defined in clause 9.5 of ETSI
EN 302 755 [1]. Each sample value shall be sent as a 12-bit two's complement value I for the real part immediately
followed by a 12-bit two's complement value Q for the imaginary part of the complex number. The sample value,
pFEF(t), shall be given by:

Re(

p

FEF

t
))(

=

Im(

p

FEF

t
(

))

=

I
9
2
Q
9
2

where I and Q are the 12-bit two's complement values represented as integers in the range -211 to 211-1. The transmitted
signal s(t) during the T2 frames is defined in clause 9.5 of ETSI EN 302 755 [1], and the signal during the FEF parts,
using the same scaling, shall be given by:

ts
)(

=

{
e
 Re

π
2
j

tf
c

})(
t

p
FEF

ETSI

28

ETSI TS 102 773 V1.4.1 (2016-03)

NOTE:  This allows a peak modulation magnitude, with any phase, of 12dB above the rms level of the signal

during the T2-frames. The quantization noise is approximately 59dB below the rms level of the
T2-frames.

When this T2-MI packet type is used, the mean power of the complex samples E(|pFEF|2) shall not exceed unity.

If more than one T2-MI packet is used for a particular FEF part, the payload of T2-MI packets with an unfinished
stream shall end with a completed sample. The next sample value for that FEF part shall then start at the beginning of
the payload of the next T2-MI packet with packet_type 3116 with the same fef_idx. All T2-MI packets of type 3116
with a given fef_idx shall have the same value of s1_field and s2_field. The total number of complex samples shall
equal FEF_LENGTH-2048 where FEF_LENGTH is the L1-post configurable signalling field defined in clause 7.2.3.1
of ETSI EN 302 755 [1].

5.2.11  FEF part: composite

T2-MI packets with a packet_type of 3216 shall carry information related to a FEF part, in accordance with ETSI
EN 302 755 [1], clause 8.4, formed as a composite of sub-parts as depicted in figure 14.

FEF_LENGTH

P1

sub-part
index 0

sub-part
index 1

...

sub-part
index P-1

subpart_length
for sub-part 0

FEF-part

Figure 14: The division of FEF parts into sub-parts

The composite FEF part is signalled to the modulator using the T2-MI packet payload shown in figure 15. The actual
sub-parts are carried in separate packets of packet_type 3316 defined in clause 5.2.12. A packet of packet_type 3216
for a given fef_idx shall arrive at a modulator before any packets describing sub-parts. A complete set of P sub-parts
describing the entire FEF-part and whose total length adds up to FEF_LENGTH (clause 7.2.3.1 of ETSI
EN 302 755 [1]) shall be signalled to the modulator.

fef_idx

rfu1

s1_field

s2_field

rfu2

num_subparts

8 bits

1 bit

3 bits

4 bits

32 bits

16 bits

Figure 15: FEF part: composite payload

fef_idx (8 bits) indicates the index of the FEF part within the super-frame. The first FEF part in a super-frame shall
have a fef_idx value of "0" and this shall increment by 1 for each subsequent FEF part.

rfu1 (1 bit) is reserved for future use and shall be set to 02.

s1_field gives the value of the S1 field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

s2_field gives the value of the S2 field in the P1 preamble of the FEF part according to clause 7.2.1 of ETSI
EN 302 755 [1].

rfu2 (32 bits) reserved for future use and shall all be set to 02.

num_subparts (16 bits) signals the total number of sub-parts P making up the FEF part.

The overall composition of the sub-parts as defined by this packet type shall be the same for all modulators fed by a
single T2-MI feed. However, the contents of individual sub-parts may be addressed to modulators or combinations of
modulators individually by means of a tx_identifier field in the FEF sub-part (see clause 5.2.12).

ETSI

29

ETSI TS 102 773 V1.4.1 (2016-03)

5.2.12  FEF sub-part

5.2.12.0

Introduction

T2-MI packets with a packet_type of 3316 shall carry information related to a FEF sub-part as shown in figure 16.

fef_idx

tx_identifier

rfu1

subpart_idx

subpart_variety

rfu2

subpart_length

subpart()

8 bits

16 bits

32 bits

16 bits

16 bits

10 bits

22 bits

variable

Figure 16: FEF part: sub-part payload

fef_idx (8 bits) indicates the index of the FEF part within the super-frame. The first FEF part in a super-frame shall
have a fef_idx value of 0 and this shall increment by 1 for each subsequent FEF part. The fef_idx shall be the same for
all the sub-parts that form part of the same FEF part.

tx_identifier (16 bits) is a word used to address a sub-part to individual transmitters or modulators. This field has the
same meaning as in clause 5.2.8. A value of 000016 is used as a broadcast address to address all transmitters or
modulators in the network. If a modulator receives more than one sub-part addressed to it with a given value of fef_idx
and subpart_idx, the modulator shall use the last sub-part received to form the transmitted signal.

rfu1 (32 bits) are reserved for future use and shall all be set to 02.

subpart_idx (16 bits) indicates the sub-part index p of the sub-part that makes up the FEF part according to
clause 5.2.11. Sub-parts shall be assembled in order or increasing sub-part index.

rfu2 (10 bits) are reserved for future use and shall all be set to 02.

subpart_length (22 bits) signals the length in elementary time periods of this sub-part. The length of all the sub-parts
with a given fef_idx shall add up to FEF_LENGTH-2048.

subpart_variety (16 bits) indicates the variety of the FEF sub-part. A number of different varieties of FEF sub-part
have been defined as shown in table 13.

Table 13: Signalling of subpart_variety

subpart_variety value
000016
000116
000216
000316
000416 to FFFF16

FEF sub-part variety
Null
IQ
PRBS
TX-SIG: FEF
Reserved for future use

subpart() is a field whose format and length varies depending on the signalled value of subpart_variety. The format of
the field is detailed in the clauses that follow.

5.2.12.1

FEF sub-part: Null

This sub-part variety instructs a modulator to transmit a null sub-part during which no signal shall be generated by the
modulator. Its format is described in table 14.

Table 14: FEF sub-part: Null

Format

Number of bits

Format

subpart() {

reserved_for_future_use

32

bflbf

}

reserved_for_future_use (32 bits) are reserved for future use and shall all be set to 02.

ETSI

30

ETSI TS 102 773 V1.4.1 (2016-03)

5.2.12.2

FEF sub-part: IQ

This sub-part variety instructs a modulator to transmit a set time-domain IQ samples for the duration of the sub-part. Its
format is described in table 15.

Table 15: FEF sub-part: IQ data

Format

Number of bits

Format

subpart() {

reserved_for_future_use
iq_data

32
variable

bflbf
(see below)

}

reserved_for_future_use (32 bits) are reserved for future use and shall all be set to 02.

iq_data (variable bits) carries the IQ data for the FEF sub-part. It shall consist of the complex sample values in time
order, starting from the first sample after the end of the P1 preamble, at a sampling rate of 1/T as defined in clause 9.5
of ETSI EN 302 755 [1]. Each sample value shall be sent as a 12-bit two's complement value I for the real part
immediately followed by a 12-bit two's complement value Q for the imaginary part of the complex number. The sample
value, pFEF(t), shall be given by:

Re(

p

FEF

t
))(

=

Im(

p

FEF

t
(

))

=

I
9
2
Q
9
2

where I and Q are the 12-bit two's complement values represented as integers in the range -211 to 211-1. The transmitted
signal s(t) during the T2 frames is defined in clause 9.5 of ETSI EN 302 755 [1], and the signal during the FEF parts,
using the same scaling, shall be given by:

ts
)(

=

{
e
 Re

π
2
j

tf
c

})(
t

p
FEF

NOTE:  This allows a peak modulation magnitude, with any phase, of 12dB above the rms level of the signal

during the T2-frames. The quantization noise is approximately 59dB below the rms level of the
T2-frames.

When this T2-MI packet type is used, the mean power of the complex samples E(|pFEF|2) shall not exceed unity.

If it is required to convey more IQ sample data than can be conveyed in a single T2-MI packet then the time sample
data shall be split across more than one sub-part.

5.2.12.3

FEF sub-part: PRBS

This sub-part variety instructs a modulator to transmit a sub-part containing data generated by a PRBS. Its format is
shown in table 16.

Table 16: FEF sub-part: IQ data

Format

Number of bits

Format

subpart() {

prbs_type
reserved_for_future_use

}

8
96

uimsbf
bflbf

prbs_type (8 bits) indicates the type of PRBS and technique used to generate the FEF sub-part. The allowed values are
shown in table 17.

ETSI

31

ETSI TS 102 773 V1.4.1 (2016-03)

Table 17: Signalling of subpart_variety

prbs_type
0016
0116 to FF16

FEF sub-part variety
User-defined test and measurement
Reserved for future use

reserved_for_future_use (96 bits) are reserved for future use and shall all be set to 02.

5.2.12.4

FEF sub-part: TX-SIG FEF

This sub-part variety instructs a modulator to form a sub-part during which time a Transmitter Signature using a
FEF [9] shall be transmitted. Its format is shown in table 18.

Table 18: FEF sub-part: TX-SIG using a FEF

Format

Number of bits

Format

subpart() {

reserved_for_future_use

32

bflbf

}

reserved_for_future_use (32 bits) are reserved for future use and shall all be set to 02.

NOTE 1:  At the time of writing the present document, the Tx Signature FEF is defined as an 'Undefined FEF part',
signalled by S1=010, S2=000X in the T2-MI packet with packet_type of 3216 (clause 5.2.11) describing
the FEF-part that contains this sub-part.

NOTE 2:  The particular sequence numbers used in the formation of the TX-SIG by a given transmitter or
modulator can be signalled using an individual addressing function (clause 5.2.8.2.6).

5.3

Generation of L1 signalling from the T2-MI packets

The behaviour of a DVB-T2 modulator operating with a T2-MI signal as described by the present document is defined
by the DVB-T2 specification [1] of the signal-on-air combined with the definition of the content of the various T2-MI
packets, together with certain configuration parameters for the individual modulator.

Modulators will generate the L1-pre signalling by assembling:

•

•

the L1PRE field from L1-current (type 1016) T2-MI packet having frame_idx equal to FRAME_IDX of the
T2-frame being generated; and

the CRC generated by the modulator itself.

Modulators will generate the L1-post signalling for a given T2-frame by assembling:

•

•

•

•

the L1CONF from the relevant L1-current (type 1016) T2-MI packet;

the appropriate combination of L1_DYN_CURR from the relevant L1-current (type 1016) T2-MI packet, and
L1_DYN_NEXT and L1_DYN_NEXT2 from the relevant L1-future (type 1116) T2-MI packet, as given in
table 19;

the L1_EXT from the relevant L1-current (type 1016) T2-MI packet, if present; and

the CRC generated by the modulator itself;

where the relevant packet is the one having frame_idx equal to FRAME_IDX of the T2-frame being generated.

Where a modulator is generating multiple T2 profiles (e.g. a combination of T2-Base and T2-Lite profiles), the T2 data
for each profile is carried in a self-consistent T2-MI stream for each profile.

ETSI

32

ETSI TS 102 773 V1.4.1 (2016-03)

Table 19: The combination of L1-dynamic fields used to generate the L1-post signalling

NUM_RF = 1 (non-TFS)
NUM_RF > 1 (TFS)

L1_REPETITION_FLAG=0
L1_DYN_CURR
L1_DYN_NEXT

L1_REPETITION_FLAG=1
L1_DYN_CURR, L1_DYN_NEXT
L1_DYN_NEXT, L1_DYN_NEXT2

NOTE 1:  In TFS, the L1_DYN_CURR field is never transmitted in the P2 symbols. However, the information in

this field is needed by the modulator for interleaving and frame building and so is always sent in the
L1-current T2-MI packet.

A modulator may replace the CELL_ID field in the L1-pre signalling and/or the FREQUENCY field(s) in the
configurable L1-post signalling. Modulators operating in the same Single-Frequency Network (SFN) shall all use the
same values of these fields.

NOTE 2:  If these fields are modified within a modulator this is done prior to calculation of the CRCs.

5.4

Transmission order of T2-MI packets

The T2-MI packets with packet_type 0016 (BB-Frames) for a given PLP shall be sent in the original order of the
Baseband Frames they encapsulate. The transmission of such T2-MI packets is mandatory.

The T2-MI packets with packet_type 0116 (Auxiliary streams) with a given value of aux_id and packets with
packet_type 0216 (arbitrary cells) shall be sent in the order of increasing cell address of the first cell that they carry.

The T2-MI packets with types 3016, 3116, 3216 and 3316 (FEF parts and sub-parts) shall be sent as required.

NOTE 1:  Spreading out the transmission of FEF packets over the course of a T2-frame or frames, subject to the
limits of Tmax4 and Tmin3 for a given modulator (clause 5.5), may be used to reduce the peak bit-rate
requirement of the T2-MI.

T2-MI packets of type 0016 for different PLPs and T2-MI packets of type 0116 for different values of aux_id as well as
T2-MI packets of type 0216 carrying arbitrary cell data may be multiplexed together in any order, provided the above
conditions are met.

NOTE 2:  The frame_idx in T2-MI packets of type 0016 may change at different times for different PLPs. For

example, type 0016 packets for one PLP for frame m+1 may be sent before type 0016 packets for a
different PLP for frame m. This is particularly likely when multi-frame interleaving is in use.

Immediately following the last transmitted T2-MI packet with packet_type 0016, 0116 or 0216 with a given value of
frame_idx, the following T2-MI packets shall be sent in the order set out below:

•

•

•

one T2-MI packet with packet_type 2016 (DVB-T2 timestamp) with the same value of frame_idx. The
transmission of such a T2-MI packet for each T2 frame is mandatory. Where SFN synchronization is not
required the DVB-T2 timestamp shall be null (see clause 5.2.5.1);

if required, one T2-MI packet with packet_type 1216 (P2 bias balancing cells) with the same value of
frame_idx. Where there is no requirement for P2 bias balancing cells, this packet shall not be sent;

one T2-MI packet with packet_type 1016 (L1-current data) with the same value of frame_idx. The
transmission of one such T2-MI packet per T2 frame is mandatory.

If in-band signalling, L1-repetition or TFS are used, a T2-MI packet of type 1116 (L1-future) with the same frame_idx
shall be sent at a later time.

When the T2-MI packet with packet_type 1116 (L1-future) is used it shall always be the last T2-MI packet with a given
frame_idx and the T2-MI packet of type 1016 (L1-current) shall be the second-to-last packet with a given frame_idx.
Otherwise the T2-MI packet of type 1016 (L1-current) shall be the last T2-MI packet with a given frame_idx.

Individual addressing function packets (packet_type 2116) may be sent at any time.

ETSI

33

ETSI TS 102 773 V1.4.1 (2016-03)

In the case where multi-frame interleaving is used, there may be some values of the frame_idx field that are never
signalled in the Baseband frame packets. For those values of T2 frame index that are never signalled, the other packet
types that do signal this value of frame_idx (e.g. timestamp and L1-current) shall still be sent.

To maintain an approximately constant packet-rate at the input to the modulator, these packets should be sent at
intervals of approximately:

T +

f

T
(

FEF

/

I

)

FEF

NOTE 3:  This has the effect of minimizing the value of Tmax3 that is required to be supported by modulators (see

clause 5.5).

EXAMPLE:

A case when PI=2, showing the relative timing of the T2-MI packets and the relevant values of
frame_idx (where appropriate) is illustrated in figure 17.

Baseband frames

...

0

0

0

0

2

2

2

2

4

4

4

4

0

...

T2-timestamp

L1-current

L1-future

4

2

5

3

0

4

1

5

2

0

3

1

4

2

 + T
T
F

/I

FEF

FEF

Figure 17: Recommended timing of T2-MI packets for a single PLP case with PI=2

END of EXAMPLE.

The transmission order and timing of T2-MI packets are summarized in figure 18 (clause 5.5).

Where a modulator is generating multiple T2 profiles (e.g. a combination of T2-Base and T2-Lite profiles), the T2 data
for each profile is carried in a self-consistent T2-MI stream for each profile.

5.5

Timing of T2-MI packet transmission

In this clause, Tmin1, Tmin2, Tmin3, Tmax1, Tmax2 , Tmax3 and Tmax4 represent specification values for a modulator and
should be quoted by modulator manufacturers. Network operators should design the timing of a network carrying
T2-MI taking into account the values for each of the modulators in the network.

The T2-MI packets of type 0016, 0116, 0216, 1016, 1216 and 2016 with a given frame_idx shall be sent so as to arrive at
the modulator no later than Tmin1 before the beginning of the corresponding T2-frame is due for transmission.

The T2-MI packet of type 1116, if used, with a given frame_idx, shall be sent so as to arrive at the modulator no later
than Tmin2 before the beginning of the corresponding T2-frame is due for transmission.

T2-MI packets of type 3016, 3116, 3216 and 3316 with a given fef_idx shall arrive no later than Tmin3 before the
corresponding FEF part is due for transmission.

T2-MI packets of type 0016 with a given frame_idx shall arrive no earlier than TIF+Tmax1 before the beginning of the
corresponding T2-frame is due for transmission, where:

)(
iT
IF

=

)(
iP
I

×

I

jump

×

)(
i

+

T
F

⎛
⎜⎜
⎝

⎞
⎟⎟
⎠

T
I

FEF

FEF

is the duration of one Interleaving Frame for the corresponding PLP i.

ETSI

34

ETSI TS 102 773 V1.4.1 (2016-03)

T2-MI packets of type 0116 and 0216 with a given frame_idx shall arrive no earlier than Tmax2 before the beginning of
the corresponding T2-frame is due for transmission.

T2-MI packets of type 1016, 1116, 1216 and 2016 with a given frame_idx shall arrive no earlier than Tmax3 before the
beginning of the corresponding T2-frame is due for transmission.

T2-MI packets of type 3016, 3116, 3216 and 3316 with a given fef_idx shall arrive no earlier than Tmax4 before the
corresponding FEF part is due for transmission.

For the purposes of this clause, the time of arrival of a T2-MI packet at the modulator shall be defined as the time at
which the packet is delivered by the underlying DVB data piping protocol (see clause 6.1).

The timing and transmission order of T2-MI packets is summarized in figure 18.

packet_type

0016
(Baseband frames)

0116, 0216
(aux streams, arbitrary cells)

frame_idx=n

frame_idx=n

frame_idx=n

frame_idx=n

Tmin1

TIF + Tmax1

frame_idx=n

frame_idx=n

frame_idx=n

Tmax2

1016
(L1-current)

1216
(bias balancing cells)

2016
(T2 timestamp)

1116
(L1-future)

3016 - 3316
(FEFs)

2116
(Individual addressing)

Tmax3

frame_idx=n

frame_idx=n

frame_idx=n

Tmin2

Tmax4

fef_idx = f

fef_idx = f

fef_idx = f

Tmin3

time

Transmission
of T2 frame n

Transmission
of FEF-part, f

NOTE 1:  All operations on frame_idx are modulo NT2.
NOTE 2:
NOTE 3:  The T2-Timestamp refers to the transmission time of the super-frame, although it is sent every frame.
NOTE 4:  The figure shows a single PLP. When using multiple PLPs, TIF, and hence the timing requirements for the

Individual addressing functions (packet_type 2116) may be sent at any time.

type 0016 packets, can be different for different PLPs.

Figure 18: Timing and transmission order of T2-MI packets

Where a modulator is generating multiple T2 profiles (e.g. a combination of T2-Base and T2-Lite profiles), the T2 data
for each profile is carried in a self-consistent T2-MI stream for each profile.

ETSI

35

ETSI TS 102 773 V1.4.1 (2016-03)

6

Transport of T2-MI packets

6.0

Introduction

The structure of the T2-MI protocol stack described in clause 4.3 allows two mechanisms for distribution; one for
traditional ASI interfaces, the other for IP based networks.

Both mechanisms rely on first inserting the T2-MI packets into DVB/MPEG-2 TS packets which can then be interfaced
to a distribution network via such interfaces as described in CENELEC EN 50083-9 [i.2].

The resulting TS can then be further encapsulated into an IP stream using the DVB IPTV standard, ETSI
TS 102 034 [5].

6.1

Encapsulation of T2-MI packets in MPEG-2 TS

6.1.0

Introduction

The insertion of T2-MI packets into MPEG-2 TS packets shall be in accordance with ETSI EN 301 192 [4], clause 4,
"Data Piping". This mechanism allows for the insertion of data directly into the payload of MPEG-2 TS packets with
the minimum of additional overhead.

6.1.1

Description

The T2-MI packets are inserted, one after another, into the payload of MPEG-2 TS packets. Each new T2-MI packet
shall start immediately following the previous one. A TS packet may contain more than one T2-MI packet. T2-MI
packets that are too big to fit into the payload of a single TS packet shall be split across multiple TS packets as required.

Since the length of each T2-MI packet is variable (indicated by the payload_len field in the T2-MI packet header), the
start of a TS packet's payload does not necessarily coincide with the start of a T2-MI packet. To enable synchronization
within a device receiving T2-MI, the "payload_unit_start_indicator" bit in the TS header shall be used to indicate that a
new T2-MI packet starts somewhere within the current TS packet. When this is the case an 8-bit pointer shall be
positioned as the first payload byte of the TS packet, indicating the offset from the start of the TS payload to the first
byte of the first T2-MI packet. This 8-bit pointer field (uimsbf) shall indicate the number of bytes immediately
following the pointer field until the first byte of the first T2-MI packet that is present in the payload of the Transport
Stream packet (i.e. a value of 0016 in the pointer field indicates that the T2-MI packet starts immediately after the
pointer field). This is illustrated in figure 19.

Figure 19: Encapsulation of T2-MI Packets in MPEG-2 TS

Using this mechanism the T2-MI packet can begin anywhere in the TS packet. There is no requirement to have T2-MI
packets beginning at the start of a TS packet and no need for unnecessary stuffing.

NOTE 1:  Since the TS packets containing T2-MI packets are carrying a data type not defined by MPEG,

ETSI EN 301 192 [4] allows the use of the "payload_unit_start_indicator" bit in this "service private
way".

When a T2-MI packet ends at the last-but-one byte of a TS packet and starts in a previous TS packet, the one remaining
byte does not allow space for both the insertion of the 8-bit pointer field and the first byte of the next T2-MI packet. In
this case the size of the payload of the TS packet shall be reduced by one byte through the use of adaptation field
stuffing [7] such that the current T2-MI packet finishes at the end of the TS packet payload. The next T2-MI packet
shall start in the next TS packet having the same PID.

ETSI

36

ETSI TS 102 773 V1.4.1 (2016-03)

NOTE 2:  Arbitrary amounts of padding may also be added, if required, at this layer through the use of arbitrary

numbers of stuffing bytes in the adaptation field of the transport stream packet [7].

EXAMPLE:

A T2-MI packet is being transmitted. Most of the T2-MI packet has been transmitted and only
50 bytes remain to be sent. The next T2-MI packet is not yet available and there are therefore not
enough bytes to fill up a TS packet. To allow this TS packet to be transmitted immediately, an
adaptation field of total length 134 bytes (adaptation_field_length = 133) containing stuffing bytes
can be inserted before the payload.

For carriage over managed distribution networks a minimum of PSI should be used in order to prevent erroneous alarms
from being set. This would normally comprise a PAT, and PMT for a single "Program" as defined in
ISO/IEC 13818-1 [7]. The Stream Type to be used in the PMT is not defined in ETSI EN 301 192 [4]. For the purposes
of interoperability, it should be set to 0616 and, if used, the T2MI_descriptor [8] shall be added to a PMT sub-table, for
every T2-MI stream.

Similarly, some networks may require the carriage of mandatory DVB SI tables, and reference should be made to ETSI
EN 300 468 [8] for the appropriate values to be used in such tables.

When NUM_RF=1, the maximum rate of the transport stream carrying the T2-MI shall be 72 Mbps.

6.2

Encapsulation of MPEG-2 TS in IP packets

6.2.0

Introduction

A DVB-T2 modulator may support the transport of MPEG-2 TS over IP. In case the DVB-T2 modulator supports
IP-based delivery, the transport of MPEG-2 TS over IP shall follow the specification in this clause. The transport of
MPEG-2 TS over IP relies on the methods specified in ETSI TS 102 034 [5]. This clause specifies a protocol for FEC
protected multicast delivery of MPEG-2 Transport Streams over RTP and is based on IP version 4 according to ETSI
TS 102 034 [5]. IP version 6 is not supported.

Unicast delivery of MPEG-2 Transport Streams over IP is outside the scope of the present document. However, the
unicast transport may rely on the same protocol as specified in clause 6.2.2.

6.2.1

Setup Information

For delivering FEC-protected, multicast MPEG-2 Transport Streams over RTP using the protocols in ETSI
TS 102 034 [5], the following setup information should be provided according to ETSI TS 102 034 [5], clause 5.2.6.2,
table 4:

•

IPMulticastAddress:

-

-

-

-

IPMulticastAddress@Source: Optionally the IP unicast address of the source of the TS may be provided.

IPMulticastAddress@Address: Provides the multicast address at which the service may be accessed.

IPMulticastAddress@Port: Provides the port at which the service may be accessed.

FECBaseLayer: Contains the multicast address and port of the AL-FEC stream. This element shall be
present if the FECBaseLayer element is present:
(cid:0)

FECBaseLayer@Address: IP Multicast Address for FEC Base Layer. If the IP multicast address is
omitted, then the FEC flow is assumed to be on the same multicast address as the original data.

(cid:0)

(cid:0)

FECBaseLayer@Source: IP Multicast Source Address for FEC Base Layer. If the IP multicast
source address is omitted, then the FEC flow is assumed to be on the same multicast source address
as the original data.

FECBaseLayer@Port: UDP port for FEC Base Layer.

ETSI

37

ETSI TS 102 773 V1.4.1 (2016-03)

-

FECEnhancementLayer: Contains the multicast address and port of the AL-FEC enhancement stream(s).
This element shall only be present if the FECBaseLayer element is present. This element may be
repeated for multiple layers:
(cid:0)

FECEnhancementLayer@Address: IP Multicast Address for FEC Enhancement Layer. If the IP
multicast address is omitted, then the FEC flow is assumed to be on the same multicast address as
the original data.

(cid:0)

(cid:0)

(cid:0)

(cid:0)

(cid:0)

FECEnhancementLayer@Source: IP Multicast Source Address for FEC Enhancement Layer. If the
IP multicast source address is omitted, then the FEC flow is assumed to be on the same multicast
source address as the original data.

FECEnhancementLayer@Port: UDP port for FEC Enhancement Layer.

IPMulticastAddress@FECMaxBlockSizePackets: This indicates the maximum number of stream
source packets that will occur between the first packet of a source block (which is included) and the
last packet for that source block (source or repair).

IPMulticastAddress@FECMaxBlockSizeTime: The maximum transmission duration of any FEC
Block (source and repair packets).

IPMulticastAddress@FECObjectTransmissionInformation The FEC Object Transmission
Information for the Raptor code. If a FECEnhancementLayer element is included then this element
shall be included.

For details of the semantics of these parameters refer to ETSI TS 102 034 [5].

6.2.2

Transport Protocols

Where the MPEG-2 TS is transported over IP, the MPEG-2 TS shall be encapsulated in RTP (Real-time Transport
Protocol) according to IETF RFC 3550 [6] as specified in ETSI TS 102 034 [5], clause 7.1.1.

RTCP sender reports and receiver reports shall not be used.

FEC protection of the MPEG-2 Transport Stream may be provided according to ETSI TS 102 034 [5], clauses E.3 and
E.4. When a DVB AL-FEC enhancement layer is provided, the FEC Scheme defined in ETSI TS 102 034 [5],
clause E.4.3.2 shall be used.

DVB-T2 modulators that support the transport of MPEG-2 TS over IP shall support the minimum decoder requirements
according to ETSI TS 102 034 [5], clause E.5.1.1, i.e. FEC decoders shall support processing of the DVB AL-FEC base
layer packets.

DVB-T2 modulators that support the transport of MPEG-2 TS over IP may support the enhanced decoder requirements
according to ETSI TS 102 034 [5], clause E.5.1.2, i.e. FEC decoders may support processing of the DVB AL-FEC base
layer and DVB AL-FEC enhancement layer packets.

6.2.3

Session Initiation and Control

Session initiation is outside the scope of the specification. The session initiation and control for the multicast
distribution according to ETSI TS 102 034 [5], clause 7.3.1 may be used.

6.2.4

Network Requirements

The network requirements for the multicast distribution shall be in accordance with ETSI TS 102 034 [5], clause 7.2.

In case application layer FEC is applied, the network requirements may be relaxed. For configuration examples of
application layer FEC for different network characteristics, refer to DVB BlueBook A115 [i.3].

ETSI

38

ETSI TS 102 773 V1.4.1 (2016-03)

Annex A (normative):
Calculation of the CRC word

The implementation of Cyclic Redundancy Check codes (CRC-codes) allows the detection of transmission errors at the
receiver side. For this purpose CRC words shall be included in the transmitted data. These CRC words shall be defined
by the result of the procedure described in this annex.

A CRC code is defined by a polynomial of degree n:

( )
xG
n

=

n

x

+

g

x

−
1

n

−
1

n

+

K

+

xg
2

2

+

xg
1

+

1

with

1≥n

:

and:

∈

{ }
, 1,0

gi

=

i

n
.....1

−

1

The CRC calculation may be performed by means of a shift register containing n register stages, equivalent to the
degree of the polynomial (see figure A.1). The stages are denoted by b0... bn-1, where b0 corresponds to 1, b1 to x, b2 to
x2,..., bn-1 to xn-1. The shift register is tapped by inserting XORs at the input of those stages, where the corresponding
coefficients gi of the polynomial are "1".

Data Input

g  1

g  2

g  n  -2

g   n  -1

LSb

b  0

b  1

b  n   -2

b  n   -1

MSb

Figure A.1: General CRC block diagram

At the beginning of the CRC-32 calculation all register stage contents are initialized to ones.

After applying the first bit of the data block (MSB first) to the input, the shift clock causes the register to shift its
content by one stage towards the MSB stage (bn-1), while loading the tapped stages with the result of the appropriate
XOR operations. The procedure is then repeated for each data bit. Following the shift after applying the last bit (LSB)
of the data block to the input, the shift register contains the CRC word which is then read out. Data and CRC word are
transmitted with MSB first.

The CRC code used in the T2-MI packet is based on the following polynomial:

xG
)(
32

=

32

x

+

26

x

+

23

x

+

22

x

+

16
x

+

12
x

+

11
x

+

10
x

+

8

x

+

7

x

+

5

x

+

4

x

+

2

x

++
x

1

NOTE:  The CRC-32 coder defined in this annex is identical to that specified in annex F of the DVB-T2 system

specification [1].

ETSI

39

ETSI TS 102 773 V1.4.1 (2016-03)

Annex B (normative):
T2 Modulator Information Packet (T2-MIP)

B.1  Use of the T2-MIP for over the air synchronization

The T2-MI packets, as described in the main body of the present document, are only used by the modulator and not
broadcast from the transmitter. For use cases where several repeaters are receiving a DVB-T2 signal from a main
transmitter and retransmitting it on a common second frequency, in an SFN, there is a need to make this retransmission
from the repeaters in a time-synchronized way. This situation is detailed in figure B.1.

There are two types of repeater. They may be:

•

•

regenerative repeaters, i.e. demodulating the DVB-T2 signal and the re-modulating the demodulated transport
streams to form a regenerated DVB-T2 signal which is then retransmitted; or

transposers, i.e. they would shift frequency, amplify, delay and transmit the received DVB-T2 signal without a
full re-modulation process taking place.

Figure B.1: SFN Relays taking input over the air from the Main transmitter

In this situation, the relay transmitters do not have access to the T2-MI packets that were used by the modulator at the
main transmitter to generate the on-air, physical layer, T2 signal.

Because the physical layer signal has been defined at the main transmitter, the only synchronization data required by the
relay is the time of emission. This is carried by a special Transport Stream packet (the T2-MIP) which is carried in the
over-air DVB-T2 signal.

This TS packet can be decoded by a demodulator in each repeater to extract the required emission time of a particular
super-frame of the DVB-T2 signal. Based on this information, and on the knowledge of the timing of the
currently-received super-frame, each repeater can apply the appropriate time delay to ensure emission of the super-
frame at the required time.

This version of the T2-MI specification only defines the T2-MIP to be carried over transport streams, which is derived
from the equivalent packet used in DVB-T networks [3]. There is currently no equivalent specification for such a
mechanism to synchronize networks carrying services over other transports, such as GSE [2].

ETSI

40

ETSI TS 102 773 V1.4.1 (2016-03)

See figure B.2 for the architecture of such a network.

NOTE:  The T2-MIP inserter resides in the T2 Gateway, as it is this unit that defines the construction of the T2

Frame and Super-frame, and hence the timing relationship of TS packets to the physical layer modulation.

Figure B.2: Generic architecture of over-air distribution of the T2-MIP to a SFN Sub-network

Under this condition, it is envisaged that the receiver at the relay station would deconstruct the incoming DVB-T2
signal into the constituent parts such that it could effectively pass an equivalent of the T2-MI signal on to the relay's
modulator. This is necessary to ensure that every relay's modulator constructs the air interface identically at each station
in the SFN.

Where the network is broadcasting multiple T2 Profiles (e.g. a combination of T2-Base and T2-Lite profiles), accurate
reconstruction of the physical layer signal will need to be maintained for all profiles.

B.2

T2-MIP Definition

B.2.1  Field description

The T2-MIP is an MPEG-2 compliant Transport Stream (TS) packet [7], made up of a 4 byte header and 184 data bytes.
The organization of the T2-MIP is shown in table B.1.

ETSI

41

ETSI TS 102 773 V1.4.1 (2016-03)

Table B.1: DVB-T2 Modulator Information Packet (T2-MIP)

Syntax

Number of bits

Identifier

t2_modulator_information_packet() {

transport_packet_header
synchronization_id
section_length
t2_timestamp_mip_length
t2_timestamp_mip
rfu_length
for i = 1..rfu_length {

rfu_byte

}
individual_addressing_length
for j = 1..individual_addressing_length {

individual_addressing_byte

}
crc_32
for k = 1..stuffing_length {

stuffing_byte

}

32
8
8
8
88
8

8

8

8

32

8

bslbf
uimsbf
uimsbf
uimsbf
bslbf
uimsbf

uimsbf

uimsbf

uimsbf

rpchof

uimsbf

}
NOTE 1:   Optional parameters are shown in italic.
NOTE 2:   The total length of a T2-MIP shall always be 188 bytes.

transport_packet_header (32 bits) shall comply with ISO/IEC 13818-1 [7], clause 2.4.3.2, tables 3 and 4.

•

•

•

•

•

•

•

The PID value for the T2-Modulator Information Packet (T2-MIP) shall be 1516.

The payload_unit_start_indicator is not used by the SFN synchronization function and shall be set to 1.

The transport_priority value is not used by the SFN synchronization function and shall be set to 1.

The transport_scrambling_control value shall be set to 00 (not scrambled).

The adaptation_field_control value shall be set to 01 (payload only).

All other parameters are according to ISO/IEC 13818-1 [7], clause 2.4.3.2.

The Transport Packet Header (TPH) is mandatory.

Mandatory T2-MIP fields

synchronization_id (8 bits) is used to identify the synchronization scheme used. For DVB-T2 the value shall be 0216.

NOTE 1:  The values of synchronization_id that apply for different transmissions systems are defined in table 2 of

ETSI TS 101 191 [3].

section_length (8 bits) specifies the number of bytes following immediately after the section_length field until, and
including, the last byte of the crc_32 but not including any stuffing_byte. The section_length shall not exceed
182 bytes.

t2_timestamp_mip_length (8 bits) specifies the length in bytes of the t2_timestamp_mip field that follows. The value
is currently fixed at 1110.

t2_timestamp_mip (88 bits) is in the identical format to that specified for the complete payload of the T2-MI packet
with packet type 2016 (see clause 5.2.7). The values expressed by this field refer to the emission time from the repeater
of the T2 super-frame in which the last bit of the payload of the TS packet carrying the T2-MIP appears.

NOTE 2:  The value of the T2 timestamp carried by the T2-MIP may be different from that contained in packet type

2016 of the T2-MI interface being used as input to the modulator of the main station.

ETSI

42

ETSI TS 102 773 V1.4.1 (2016-03)

rfu_length (8 bits) specifies the number of rfu_bytes that follow. A value of 0016 indicates that there are no following
rfu_bytes. This value is currently fixed at 0010, i.e. there are no rfu_bytes defined.

rfu_byte is one byte of a variable number of bytes that are reserved for future use, the number of which is defined by
the rfu_length field. All bytes shall have the value 0016.

individual_addressing_length (8 bits) gives the total length of the individual addressing loop in bytes. If individual
addressing of transmitters is not performed the field value is 0016 and there shall be no individual_addressing_byte
field.

individual_addressing_byte contains the bytes of the individual_addressing_data field of a T2-MI packet of type
2116 (see clause 5.2.8).

crc_32 (32 bits) is calculated across all other bits in the packet, including the transport_packet_header but excluding
the stuffing_byte field, in accordance with annex A.

stuffing_byte shall have the value FF16. There shall be a multiple of stuffing_bytes such that the
t2_modulator_information_packet is exactly 188 bytes long.

NOTE 3:  Whilst the values for the t2_timestamp field and the individual_addressing_bytes follow the format of

the payloads of T2-MI packet types 2016 and 2116 respectively, the values carried may be different to
those carried in these packets within the T2-MI.

B.2.2  Transmission of the T2-MIP over DVB-T2

The T2-MIP may be transmitted in one or more of the transport streams being sent over DVB-T2. If the T2-MIP is used
there has to be at least one complete T2-MIP within a T2 super-frame for each T2 profile. The T2-MIP may be sent at
any time within the super-frame and the timing may be different from super-frame to super-frame (see the definition of
the t2_timestamp_mip field in clause B.2.1).

Where multiple PLPs are used, only one of the PLPs should carry a T2-MIP. If it is carried in multiple PLPs then the T2
timestamp shall be identical within all PLPs for that super-frame.

NOTE:  Where a common PLP is available, this is the preferred location for the T2-MIP.

ETSI

43

ETSI TS 102 773 V1.4.1 (2016-03)

Annex C (informative):
Local Content Insertion

When carrying the data for a T2 transmission containing multiple PLPs, local content can be inserted into individual
PLPs into the T2-MI at a point or points downstream of the T2-gateway. This annex describes one way of achieving
this.

A typical application is shown in figure C.1.

Figure C.1: Local content insertion into a T2-MI stream within a T2-system

A Local Content Inserter (LoCI) takes as its input the T2-MI stream generated in the T2-gateway, inserts any local
content and outputs the resulting T2-MI stream.

For PLPs that are to carry locally inserted content, the T2-gateway performs the allocation of all BB frames as normal;
generating type 0016 T2-MI packets that are both consistent with the L1 signalling carried in type 1016 and 1116 T2-MI
packets and that have the correct timing.

For a given PLP, the LoCI filters the incoming type 0016 T2-MI packet (carrying Baseband Frames) based on the plp_id
field (see clause 5.2.1). The BBFRAME data field of every T2-MI packet with a matching plp_id field, is then replaced
with the local content, using BB frame padding as appropriate and the CRC32 field re-calculated.

Where no content is available at the T2-gateway to fill those BB frames pertaining to PLPs that are to be later replaced
in a LoCI, the BB frames may be of zero DataField Length (DFL = 0), i.e. all padding.

The LoCI can deal with the timing between the incoming T2-MI and the local transport streams at the input to the LoCI
in a number of ways. There are three possibilities:

•

•

•

The local transport stream is locked to the T2-MI and is at a rate that exactly matches that needed to fill the BB
frames to be replaced.

The local transport stream rate is lower than that needed to exactly fill the BB frames to be replaced and BB
frame padding is inserted by the LoCI.

The local transport stream rate is lower than that needed to exactly fill the BB frames to be replaced and Null
TS packets are inserted into the BB frames by the LoCI. In this case the LoCI performs any necessary
restamping.

This method of local content insertion has the advantage that the LoCI can be a simple device that does not require any
knowledge of the SFN timestamp. The disadvantage is that capacity is allocated on the link carrying the original T2-MI
from the T2-gateway for BB frames that are to be replaced with local content.

ETSI

44

ETSI TS 102 773 V1.4.1 (2016-03)

Annex D (informative):
MISO Management

As described, the T2-MI is designed to ensure that modulators in a network all generate identical signals at identical
times. When the MISO option of DVB-T2 is used, as described in clause 9.1 of ETSI EN 302 755 [1], the modulators
belonging to transmitter group 1 are required to generate different signals to those in transmitter group 2. Nevertheless,
all modulators in the network carry identical Baseband Frames and L1-signalling, with identical timing, so the same
T2-MI stream is therefore sufficient for all modulators. In addition, each modulator needs to be configured as belonging
to either group 1 or group 2. This can be seen as another modulator-specific parameter, similar to the power, frequency
or individual time offset, and may be configured locally at the modulator, by a central control system, or using the
individual addressing function described in clause in the described in clause 5.2.6.

ETSI

45

ETSI TS 102 773 V1.4.1 (2016-03)

Annex E (informative):
T2-MI overhead

E.0

Introduction

This annex gives an indication of the overhead associated with the encapsulation of T2 data within T2-MI packets and
the additional overhead involved in the transport of T2-MI over MPEG-2 Transport Stream or IP both with and without
the use of Forward Error Correction (FEC).

E.1

Encapsulation of T2 data within T2-MI packets

The encapsulation of T2 data within T2-MI packets (clause 5.1) requires an overhead due to:

•

•

•

the T2-MI header (6 bytes);

the crc32 field (4 bytes); and

the additional fields within the T2-MI packet payload associated with the carriage of BB frames (3 bytes).

For a typical T2 configuration (as given in clause 4.3 of ETSI TS 102 831 [i.1]), the payload of the BB-frames is
Kbch = 38 688 bits.

Excluding L1-signalling and assuming that there is no timestamp or auxiliary stream information, the overhead
associated solely with carriage of the Baseband Frame data over T2-MI is:

13 / (38 688 / 8) × 100 = 0,27 %.

E.2

Transport of T2-MI packets

E.2.1  T2-MI packets over MPEG-2 TS

E.2.1.0

Introduction

Encapsulation of T2-MI packets within 188-byte MPEG-2 TS packets using "data piping" (clause 6.1) requires an
overhead due to:

•

the TS header (4 bytes).

Assuming that the overhead due to the pointer to the start of the T2-MI packet is negligible, the resulting overhead is
therefore 4 / (188 - 4) = 2,2 %.

NOTE:  This value does not take into account any null packets inserted to keep the TS bit rate constant and any
additional PSI/SI information to be compliant with ETSI TR 101 290 [i.4]. The contribution of at least
one PAT table and one PMT table with a data broadcast descriptor is assumed to be negligible.

E.2.1.1  FEC overhead for an ASI link

Where FEC is required on a physical ASI link carrying the T2-MI packets over MPEG-2 TS, RS(188,204) can be used.
This results in an additional 8,5 % overhead.

The total overhead (T2-MI packets over ASI plus FEC) is (16 + 4) / (188 - 4) = 10,9 %.

ETSI

46

ETSI TS 102 773 V1.4.1 (2016-03)

E.2.2  T2-MI packets over MPEG-2 TS to IP

E.2.2.0

Introduction

Encapsulation of TS streams in RTP/UDP/IP packets according to clause 6.2 results in an overhead due to:

•

•

•

the RTP header (12 bytes);

the UDP header (8 bytes); and

the IP header (20 bytes) (without options).

Typically, a maximum of 7 MPEG-2 TS packets are encapsulated into one IP packet to remain below the Ethernet MTU
and hence avoid fragmentation.

The resulting overhead is therefore: 40 / (188 × 7) = 3 %.

The total overhead for T2-MI packets over MPEG-2 TS to IP is (40 + 4 × 7) / (184 × 7) = 5,3 %.

When the Ethernet header is taken into account, the total overhead for T2-MI packets over MPEG-2 TS to IP is
(40 + 18 + 4 × 7) / (184 × 7) = 6,7 %.

E.2.2.1  FEC overhead

The additional overhead due to the FEC schemes defined in ETSI TS 102 034 [5] can vary a great deal depending on
the chosen FEC profile. As an illustration two cases are considered below.

For a 1-D SMPTE 2022-1 code with 20 columns:

The additional overhead is (40 + 12 + 188 × 7) / (20 × (40 + 188 × 7)) = 5 %.

The total overhead for T2-MI packets over MPEG-2 TS to IP with FEC is
20 × (40 + 4 × 7) + (40 + 12 +188 × 7) / (20 × (184 × 7)) = 10,6 % (12,1 % including Ethernet header).

For a 1-D SMPTE 2022-1 code with 4 columns:

The additional overhead is (40 + 12 + 188 × 7) / (4 × (40 + 188 × 7)) = 25,2 %.

The total overhead for T2-MI packets over MPEG-2 TS to IP with FEC is
(4 × (40 + 4 × 7) + (40 + 12 + 188 × 7) / (4 × (184 × 7)) = 31,8 % (33,7 % including Ethernet header).

E.3

Summary of the overheads associated with T2-MI

The total overhead due to the encapsulation of T2-MI packets over ASI or Ethernet physical layers is shown in
table E.1.

Table E.1: Summary of the T2-MI overhead when transported over different physical layers

Physical layer

Total overhead

FEC scheme

ASI
Ethernet

2,2 %
6,7 %

RS(188,204)
1-D SMPTE, 20 column
1-D SMPTE, 4 column

Additional overhead
due to FEC
8,5 %
5 %
25,2 %

Total overhead
including FEC
10,9 %
12,1 %
33,7 %

ETSI

47

ETSI TS 102 773 V1.4.1 (2016-03)

Annex F (informative):
DVB-T2 Timestamps

F.1  Relationships

The relationships between UTC, TAI, GPS Time and the DVB-T2 Timestamp (as defined in clause 5.2.2) are, as at the
time of writing (February 2009), as follows:

•

•

•

•

•

•

•

GPS = TAI - 19 s (constant).

UTC = TAI - 34 s (variable due to leap seconds).

UTC = GPS - 15 s (variable due to leap seconds).

UTC = DVB-T2 - utco (constant due to varying value of utco).

DVB-T2 = TAI - 32 s (constant).

DVB-T2 = GPS - 13 s (constant).

DVB-T2 = UTC + utco (constant due to varying value of utco).

F.2  Rationale

Several other standard time/date encodings are in common use, including MJD, UTC, GPS and TAI. It was agreed that
none of these adequately addressed the needs of a DVB-T2 system and that it was desirable to define a time format
specifically for the DVB-T2 Timestamp. The following reasons were given for rejecting other common timebases:

•  MJD is subject to leap seconds making the fractional portion very hard to represent in a fixed-point format.

•

•

•

UTC is subject to leap seconds making the number of seconds in a day variable (86 399 / 86 400 / 86 401).

GPS Time is subject to "week number wrapping" approximately every 19,7 years.

UTC, TAI, MJD and GPS Time all have epochs (start dates) partway through the 400-year leap-year cycle.

The DVB-T2 Timestamp is not subject to leap seconds but contains sufficient extra information (in the utco field) to
trivially convert the value to UTC which does include leap-seconds. Conversion to GPS Time and/or TAI is also trivial,
simply involving the subtraction of a constant value. The epoch for DVB-T2 Time is synchronized with the start of a
400-year leap-year cycle, making leap-year calculations simpler and less error prone.

ETSI

48

ETSI TS 102 773 V1.4.1 (2016-03)

Annex G (informative):
Use of T2-MI in Test and Measurement Setups

G.1

Introduction

T2-MI has been designed to be the real-time interface from a T2-gateway and a T2 modulator between which
synchronization can be maintained using a GPS signal.

Since the T2-MI provides an unambiguous representation of a T2 transmission, it is also useful as an intermediate
storage format for later use by test generators. This is particularly the case where multiple PLPs and dynamic allocation
are used. As an example, T2-MI stored in a TS file could be read directly by a file-based ASI player connected to a
modulator under test as shown in figure G.1.

File-player

Modulator

T2-MI over ASI

T2-MI File

To ensure the interoperability between file player and modulator:

Figure G.1: T2-MI file player

•

•

the file-player schedules the playout for each of the Transport Packets (TPs) with T2-MI payload; and

the modulator synchronizes with the T2-MI stream in the case that no synchronization source is available.

This annex describes a method to that enables this interoperability to take place.

G.2  Use of Program Clock Reference (PCR) timestamps

G.2.0   Introduction

This method uses the ISCR [1] as a shared reference clock and uses Program Clock Reference (PCR) [7] timestamps to
convey the ISCR from T2-MI transmitter to T2-MI receiver. In general, this method is similar to and compatible with
the synchronization of decoders with encoders in MPEG-2 Transport Streams, using PCR timestamps. This is defined in
ISO/IEC 13818-1 [7].

G.2.1   Relation between ISCR and PCR

To keep the method compatible with MPEG-2 Systems, the PCRs in this method are based on a 27-MHz clock.
Conversion between ISCR and PCR clock values is possible since the ratio between PCR and ISCR-clock frequency is
exact and can be expressed as an ratio with an integer numerator and denominator as shown in table G.1.

Table G.1: Bandwidths and the ratio between PCR and ISCR-clock frequency

Bandwidth
TISCR /TPCR

1,7 MHz
27 × 71/131

5 MHz
27 × 7/40

6 MHz
27 × 7/48

7 MHz
27 × 1/8

8 MHz
27 × 7/64

10 MHz
27 × 7/80

A block diagram of a T2-gateway that is able to insert PCRs into the output T2-MI is shown in figure G.2.

ETSI

49

ETSI TS 102 773 V1.4.1 (2016-03)

Figure G.2: PCR insertion in the T2-gateway

The T2-gateway is responsible for inserting the PCRs at such places that the timing of the T2-MI packet transmission
can be reconstructed through the methods described below. The output T2-MI can then be recorded to a file.

NOTE:

If the ISCRs and PCRs are generated from a common clock, this is signalled in the T2MI_descriptor [8]
carried in the PMT, when present.

G.2.2

Insertion of PCRs

The PCR values are inserted into the Transport Stream carrying the T2-MI packets. The suggested method is to insert
the PCRs on the PID that carries the T2-MI packets. Where multiple T2 profiles (T2-MI streams) are being used, a PCR
may be inserted for each profile on the unique profile PID. This applies both to the case where a single TS is used for
both profiles and for the case when different TSs are used. Alternatively a different PCR-only PID could be used if
required in both the single profile and multiple profile cases.

If the Transport Stream comprises a PAT and PMT, the PCR PID should be defined in the PMT [7].

G.2.3  Playout of a Constant Bit-rate (CBR) T2-MI file

A file player can use the PCRs in the T2-MI file to obtain an accurate estimation of the TS bitrate. This can be
determined by dividing the number of bits between two PCR values by the difference of those two PCR values.

NOTE:  The difference between the original T2-MI stream bitrate and the playout rate (due to rounding in bitrate

estimation and clock deviations) will appear as an ISCR-clock deviation in the modulator.

G.2.4  Playout of a Variable Bit-rate (VBR) T2-MI file

In the case of a VBR T2-MI file, the PCRs can be used to determine the intended transmit time of individual Transport
Stream packets that carry a PCR value. In between two Transport Stream packets carrying PCR values, the TS rate can
be considered constant and the transmit time of the packets can be interpolated linearly. This results in a VBR T2-MI
stream that is piecemeal CBR.

ETSI

50

ETSI TS 102 773 V1.4.1 (2016-03)

G.2.5  Synchronization between T2-Gateway and Modulator

The ratio between the PCR- and ISCR clocks is known exactly. If the delay between T2-gateway and modulator is
constant, the modulator can extract the PCR values and use them to synchronize its ISCR clock directly. In practice the
network will introduce some variable delay (jitter) and therefore some form of control loop will be required.

A block diagram of a modulator that is able to synchronize its ISCR-clock based on the PCR values in the T2-MI is
shown in figure G.3.

T2-MI

PCR
Extractor

OFDM
Generator

RF

Clock
Control

ISCR

Modulator

Figure G.3: ISCR synchronization in the modulator based on PCR values

ETSI

51

ETSI TS 102 773 V1.4.1 (2016-03)

Annex H (normative):
T2-MI for Composite Signals

H.1

Introduction

The introduction of the T2-Lite profile in v1.3.1 of the DVB-T2 system specification [1] allows for the T2-Lite signal to
be multiplexed together with a T2-base signal (and/or other signals), with each signal being transmitted in the other's
FEF part. An example of such a composite T2-Base/T2-Lite signal is shown in figure H.1.

To facilitate the generation of composite signals, a T2 modulator capable of generating such composite signals accepts
multiple T2-MI streams as input where each stream describes a single T2 profile.

Figure H.1: An example composite signal containing T2-Base and T2-Lite

Since there shall be no overlap between the signals pertaining to the different profiles, the relative timing needs to be
specified precisely.

This annex explains how T2-MI shall be used to carry the data for composite signals to modulators in an unambiguous
way in both SFN and MFN environments.

H.2  Multiple T2-MI Streams

A single T2-MI stream shall convey the data for the transmission of a single T2 profile, plus optionally any non-profile
data for the FEF, within the composite signal. Where a modulator is capable of transmitting a composite signal, made
up of multiple T2 profiles, it shall accept at its input multiple T2-MI streams. Each T2-MI stream that makes up the
composite signal shall be uniquely identified by a different value of t2mi_stream_id in the T2-MI packet header as
described in clause 5.1.

Where the multiple T2-MI streams are carried in a single Transport Stream, they shall be carried on different PIDs. If a
PMT is present in the TS, and the T2MI_descriptor [8] is used, it shall be added to a PMT sub-table, for every T2-MI
stream.

Each of the individual T2-MI streams that makes up the composite signal shall be self-consistent and carry all required
packet types such that a modulator that accepts a single T2-MI stream can generate a valid on-air T2 signal from that
single stream. Each individual T2-MI stream shall carry null data in those parts of the FEF where another T2 profile
(carried in another T2-MI stream) is to be emitted.

H.3  Alignment of the profiles in the emitted composite

signal

A modulator that generates a composite signal shall use the difference between the emission times calculated from the
T2 timestamps carried in the individual T2-MI streams to align the output for each T2 profile. When multiple T2-MI
streams are used to generate a composite signal, the null timestamp shall not be used.

For an MFN modulator, it is only the difference between the timestamps in each of the T2-MI streams that is required to
perform the necessary alignment of each profile. For an SFN modulator, it shall also obey the T2 Timestamp to define
the time of emission of the composite signal.

NOTE 1:  It is recommended that when using multiple T2-MI streams to generate a composite signal, the absolute

timestamp is used. This applies for both MFN and SFN modulators.

ETSI

52

ETSI TS 102 773 V1.4.1 (2016-03)

Conceptually, the modulator can be thought of as summing the outputs together, having aligned the output derived from
each of the T2-MI streams first. This results in some or all of the existing null sections of the FEF-parts being
overwritten with data from another profile.

At the start of a FEF-part, there may be a clash between the P1 symbol signalled at the start of the Null FEF-part and the
P1 symbol pertaining to the particular T2 profile. In this instance, the P1 of the T2 profile shall always take priority.
This is illustrated in figure H.2.

NOTE 2:  The P1 of the Null FEF-part of a T2 profile is either exactly aligned with the P1 of another T2 profile or

does not overlap at all. In the first instance the priority principle, as described above, is trivial. In the
second instance, there is no clash and so the conceptual summing of the outputs works correctly.

Figure H.2: Alignment of two profiles within a composite modulator

Any other clashes, for example between non-null sections of FEF-parts and T2 signals, shall be interpreted as an error
by the modulator.

NOTE 3:  Where there is a mis-configuration resulting in a clash between the different profiles, modulators might

be able to identify one T2 profile as having priority over the others. In this case, it is this profile that
would be transmitted at the expense of any others.

NOTE 4:  According to [1], the minimum interval between two P1 symbols is 10 000T.

ETSI

53

ETSI TS 102 773 V1.4.1 (2016-03)

Annex I (informative):
T2-MI for Composite Signals: Network Topology and
Synchronization

I.1

Introduction

A number of different network topologies are possible when multiple T2-MI streams are used to generate a composite
(e.g. T2-Base/T2-Lite) transmission. Since the T2-MI streams fit together exactly in order to generate the composite
output signal, there is an exact relationship between the timestamps in each stream.

This annex examines the issue of synchronization and timing and provides informative information for manufacturers.

I.2

Network Topology

A single T2-Gateway may be used to generate the multiple T2-MI streams (figure I.1) in which case the necessary
synchronization between the timestamps can be handled internally.

Figure I.1: Using a single T2-Gateway to generate multiple T2-MI streams

Alternatively multiple T2-Gateways, which may be geographically separated, may be used to generate the T2-MI for
the different profiles (figure I.2) in which case some form of external synchronization mechanism will be required.

ETSI

54

ETSI TS 102 773 V1.4.1 (2016-03)

Figure I.2: Using multiple T2-Gateways to generate multiple T2-MI streams

The multiple T2-MI streams may be multiplexed together into a single Transport Stream for distribution. Alternatively,
they may be carried on separate transport streams where a modulator provides multiple T2-MI stream inputs. These
could either be on separate ASI inputs or use different IP multicast groups/ports in the case of IP encapsulation (see
clause 6.2).

Care should be taken to ensure that the data for each T2-MI stream arrives at the modulator at such a time as to obey the
timing rules laid down in clause 5.5.

I.3

Synchronization of Multiple T2-Gateways

I.3.0

Introduction

Where multiple, independent T2-Gateways are used to generate the T2-MI streams, some form of external
synchronization mechanism will be required in order to ensure that the streams for the different T2 profiles contain
co-ordinated timestamps such that they can be correctly aligned in a modulator.

It is assumed that all the Gateways generate absolute timestamps and share a common clock source.

For each individual T2-MI stream, the sequence of super-frames is regular, with a constant distance between them.
Once the absolute starting time of a super-frame for one profile is known, the start times of all the others are also
known. One approach is to assign a priority to each of the T2-Gateways such that one Gateway (GW1) has the highest
priority, whilst other Gateways generating T2-MI streams for other profiles have lower priorities (e.g. GW2 and GW3).

The Gateway with the highest priority (GW1) begins the process and arbitrarily defines the time of emission of its
super-frames, within the normal operating constraints. A "timestamp message" is then sent to the other Gateways (GW2
and GW3), indicating to them the absolute time of emission of one super-frame. It is not necessary for GW1 to indicate
to which super-frame this refers.

Since GW2 has the same timing reference and clock, it can use this information, along with knowledge of the length of
super-frames generated by GW1, to deduce the absolute starting time of all super-frames from GW1. It will then adjust
the timestamp defining its own super-frame start time to have the desired relationship of its profile relative to the profile
generated by GW1. It is expected that this can be done in a sample accurate way. Since the super-frame lengths of GW1
and GW2 may not be identical, the difference in timestamps indicating the start of the super-frame for each profile may
vary over time.

ETSI

55

ETSI TS 102 773 V1.4.1 (2016-03)

Once GW2 is ready and is producing its own T2-MI stream, GW2 can send a message to GW3 regarding the emission
time of one of its super-frames. GW3 can then proceed in the same way as GW2. Since GW3 has the lowest priority, it
has no need to send any message. If necessary, this priority scheme can be extended to more than three profiles/levels.

Where a Gateway is designed to operate in such an environment, it should provide a mechanism for receiving and
sending the "timestamp message" from/to other Gateways. It should also provide a configuration item to indicate the
length of the super-frame of the Gateway with higher priority and a configuration item to indicate the desired offset of
the start of its super-frame relative to those generated by the Gateway of higher priority.

I.3.1   Configuration Changes and Multiple T2-Gateways

Where a change in the T2 configuration (i.e. the contents of the fields of the L1-pre signalling or the configurable part
of the L1-post signalling) that affects the layout of the T2 super-frame, this will need to be coordinated between the
Gateways. An example of such a configuration change is when the proportion of T2-Lite relative to the T2-Base might
change at various points throughout the day, including the case where there is no T2-Lite or no T2-Base at all.

When this occurs, the configuration change is signalled in advance according to the L1_CHANGE_COUNTER
mechanism specified in clause 7.2.3.2 of [1], except for the case where there is a transition from a state with no
transmission of the relevant T2 profile to an active state, due to the absence of L1 signalling previously. In the case of
multiple Gateways, this reconfiguration will also need to be coordinated between Gateways to ensure each Gateway's
configuration changes at exactly the appropriate time to ensure that a consistent set to T2-MI streams is delivered to
modulators over the boundary of the configuration change.

ETSI

56

ETSI TS 102 773 V1.4.1 (2016-03)

Annex J (informative):
Change History

Date
September 2009
December 2010
January 2012

Version
1.1.1
1.2.1
1.3.1

February 2016

1.4.1

Information about changes

New freq_source field introduced in clause 5.2.4.
A Note 3 about the use of "CURRENT_RF_IDX" for TFS added in clause 5.2.4.
New function_tag value for Frequency function added to table 6 in clause 5.2.8.2.0.
New Clause 5.2.8.2.7 added specifying Frequency function.
Editorial re-arrangements to avoid hanging paragraphs.
New Change History annex J added.

ETSI

57

ETSI TS 102 773 V1.4.1 (2016-03)

History

V1.1.1

V1.2.1

V1.3.1

V1.4.1

September 2009

Publication

December 2010

Publication

January 2012

Publication

March 2016

Publication

Document history

ETSI


