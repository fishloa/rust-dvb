ETSI EN 302 307-2 V1.4.1 (2024-08)

EUROPEAN STANDARD

Digital Video Broadcasting (DVB);
Second generation framing structure, channel coding and
modulation systems for Broadcasting,
Interactive Services, News Gathering and
other broadband satellite applications;
Part 2: DVB-S2 Extensions (DVB-S2X)

2

ETSI EN 302 307-2 V1.4.1 (2024-08)

Reference
REN/JTC-DVB-410-2

Keywords
BSS, digital, DVB, modulation, satellite, TV

ETSI

650 Route des Lucioles
F-06921 Sophia Antipolis Cedex - FRANCE

Tel.: +33 4 92 94 42 00   Fax: +33 4 93 65 47 16

Siret N° 348 623 562 00017 - APE 7112B
Association à but non lucratif enregistrée à la
Sous-Préfecture de Grasse (06) N° w061004871

Important notice

The present document can be downloaded from the
ETSI Search & Browse Standards application.

The present document may be made available in electronic versions and/or in print. The content of any electronic and/or
print versions of the present document shall not be modified without the prior written authorization of ETSI. In case of any
existing or perceived difference in contents between such versions and/or in print, the prevailing version of an ETSI
deliverable is the one made publicly available in PDF format on ETSI deliver.

Users should be aware that the present document may be revised or have its status changed,
this information is available in the Milestones listing.

If you find errors in the present document, please send your comments to
the relevant service listed under Committee Support Staff.

If you find a security vulnerability in the present document, please report it through our
Coordinated Vulnerability Disclosure (CVD) program.

Notice of disclaimer & limitation of liability

The information provided in the present deliverable is directed solely to professionals who have the appropriate degree of
experience to understand and interpret its content in accordance with generally accepted engineering or
other professional standard and applicable regulations.
No recommendation as to products and services or vendors is made or should be implied.
In no event shall ETSI be held liable for loss of profits or any other incidental or consequential damages.

Any software contained in this deliverable is provided "AS IS" with no warranties, express or implied, including but not
limited to, the warranties of merchantability, fitness for a particular purpose and non-infringement of intellectual property
rights and ETSI shall not be held liable in any event for any damages whatsoever (including, without limitation, damages
for loss of profits, business interruption, loss of information, or any other pecuniary loss) arising out of or related to the use
of or inability to use the software.

Copyright Notification

No part may be reproduced or utilized in any form or by any means, electronic or mechanical, including photocopying and
microfilm except as authorized by written permission of ETSI.
The content of the PDF version shall not be modified without the written authorization of ETSI.
The copyright and the foregoing restriction extend to reproduction in all media.

© ETSI 2024.
© European Broadcasting Union 2024.
All rights reserved.

ETSI

Contents

3

ETSI EN 302 307-2 V1.4.1 (2024-08)

1

2
2.1
2.2

3
3.1
3.2
3.3

4
4.0
4.1
4.2
4.3

Intellectual Property Rights ........................................................................................................................ 8
Foreword ..................................................................................................................................................... 8
Modal verbs terminology ............................................................................................................................ 9
Introduction ................................................................................................................................................ 9
Scope .............................................................................................................................................. 10
References ...................................................................................................................................... 10
Normative references .............................................................................................................................. 10
Informative references ............................................................................................................................. 11
Definition of terms, symbols and abbreviations ............................................................................. 11
Terms ....................................................................................................................................................... 11
Symbols ................................................................................................................................................... 11
Abbreviations .......................................................................................................................................... 11
Transmission system description .................................................................................................... 12
General aspects ........................................................................................................................................ 12
System definition ..................................................................................................................................... 12
System architecture ................................................................................................................................. 12
System configurations ............................................................................................................................. 12
Subsystems specifications .............................................................................................................. 16
Mode adaptation ...................................................................................................................................... 16
General aspects .................................................................................................................................. 16
Input Interfaces .................................................................................................................................. 16
Input stream synchronizer (optional, not relevant for single TS - BS) .............................................. 16
Null-Packet Deletion (ACM and Transport Stream only) ................................................................. 16
CRC-8 encoder (for packetized streams only) ................................................................................... 16
Merger/Slicer ..................................................................................................................................... 16
Base-Band Header insertion .............................................................................................................. 16
GSE High Efficiency Mode (GSE-HEM) .......................................................................................... 17
Channel bonding for multi-tuner (L) receivers .................................................................................. 18
Introduction to channel bonding .................................................................................................. 18
Channel bonding for TS transmission .......................................................................................... 19
Channel bonding for GSE transmission ....................................................................................... 20
General aspects ....................................................................................................................... 20
Channel bonding for Generic Packetized streams .................................................................. 22
Channel bonding for Generic Continuous streams ................................................................. 22
Stream Adaptation ................................................................................................................................... 22
General aspects .................................................................................................................................. 22
Padding .............................................................................................................................................. 22
BB scrambling ................................................................................................................................... 22
FEC Encoding ......................................................................................................................................... 22
General aspects .................................................................................................................................. 22
Outer encoding (BCH) ....................................................................................................................... 24
Inner encoding (LDPC) ..................................................................................................................... 24
General aspects ............................................................................................................................ 24
Inner coding for normal FECFRAME.......................................................................................... 24
Inner coding for short and medium FECFRAME ........................................................................ 25
Bit interleaver .................................................................................................................................... 25
Constellations and Bit mapping............................................................................................................... 26
General aspects .................................................................................................................................. 26
Bit mapping into π/2BPSK constellation (VL-SNR modes and  VL-SNR Header) .......................... 27
Bit mapping into QPSK constellation ................................................................................................ 27
Bit mapping into 8PSK and 8APSK constellations ........................................................................... 27
Bit mapping into 16APSK constellation ............................................................................................ 28
Bit mapping into 32APSK constellations .......................................................................................... 30

5
5.1
5.1.0
5.1.1
5.1.2
5.1.3
5.1.4
5.1.5
5.1.6
5.1.7
5.1.8
5.1.8.1
5.1.8.2
5.1.8.3
5.1.8.3.0
5.1.8.3.1
5.1.8.3.2
5.2
5.2.0
5.2.1
5.2.2
5.3
5.3.0
5.3.1
5.3.2
5.3.2.0
5.3.2.1
5.3.2.2
5.3.3
5.4
5.4.0
5.4.0a
5.4.1
5.4.2
5.4.3
5.4.4

ETSI

4

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.4.5
5.4.6
5.4.7
5.5
5.5.0
5.5.1
5.5.2
5.5.2.0
5.5.2.1
5.5.2.2
5.5.2.3
5.5.2.4
5.5.2.5
5.5.2.6
5.5.3
5.5.4
5.5.4.0
5.5.4.1
5.5.4.1.0
5.5.4.1.1
5.6

Bit mapping into 64APSK constellations .......................................................................................... 32
Bit mapping into 128APSK constellations ........................................................................................ 35
Bit mapping into 256APSK constellations ........................................................................................ 37
Physical Layer (PL) framing ................................................................................................................... 43
General aspects .................................................................................................................................. 43
Dummy PLFRAME insertion ............................................................................................................ 43
PL signalling ...................................................................................................................................... 44
General aspects ............................................................................................................................ 44
SOF field ...................................................................................................................................... 46
MODCOD field ............................................................................................................................ 46
TYPE field ................................................................................................................................... 48
PLS code, no time slicing ............................................................................................................. 48
VL-SNR Header ........................................................................................................................... 49
Shortening and Puncturing of VL-SNR MODCODs ................................................................... 50
Pilot Insertion..................................................................................................................................... 51
Physical layer scrambling .................................................................................................................. 51
General aspects ............................................................................................................................ 51
PL scrambling for VL-SNR frames ............................................................................................. 52
General aspects ....................................................................................................................... 52
π/2-BPSK modulated frames .................................................................................................. 52
Baseband shaping and quadrature modulation ........................................................................................ 53
Error performance .......................................................................................................................... 53

6

Annex A (normative):

Signal spectrum at the modulator output ............................................ 55

Annex B (normative):

Addresses of parity bit accumulators for nldpc = 64 800 ................... 57

Annex C (normative):

Addresses of parity bit accumulators for nldpc = 16 200 and
nldpc = 32 400 ....................................................................................... 103

Additional tools .................................................................................... 108
Annex D (normative):
D.0  General aspects ............................................................................................................................. 108
Implementation of TS based channel bonding ............................................................................. 108
D.1
Transmitting side ................................................................................................................................... 108
D.1.1
Receiving side (informative) ................................................................................................................. 108
D.1.2
D.2  Void .............................................................................................................................................. 108
D.3  Void .............................................................................................................................................. 108
D.4  Void .............................................................................................................................................. 109
D.5  Signalling of reception quality via return channel (normative for ACM) .................................... 109

Super-Framing Structure (optional) .................................................. 111
Annex E (normative):
E.1  Purpose of Super-Framing Structure ............................................................................................ 111
E.2  Specification of Super-Frame as a Container ............................................................................... 111
Super-Frame Structure .......................................................................................................................... 111
E.2.1
Start of Super-Frame (SOSF) Field ....................................................................................................... 112
E.2.2
Super-Frame Format Indicator (SFFI) Field.......................................................................................... 112
E.2.3
Two-Way Scrambling ........................................................................................................................... 113
E.2.4
General aspects ................................................................................................................................ 113
E.2.4.0
Scrambling Sequence Generation .................................................................................................... 113
E.2.4.1
Two-Way Scrambling Method ........................................................................................................ 114
E.2.4.2
E.3  Format Specifications as Super-Frame Content ........................................................................... 115
General aspects ...................................................................................................................................... 115
E.3.0
Super-Frame-aligned Pilots (SF-Pilots)................................................................................................. 116
E.3.1
General aspects ................................................................................................................................ 116
E.3.1.0
Specification of SF-Pilots Type A ................................................................................................... 117
E.3.1.1
Format Specification 0: DVB-S2X........................................................................................................ 117
E.3.2

ETSI

5

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.2.0
E.3.2.1
E.3.2.2
E.3.3
E.3.4
E.3.4.0
E.3.4.1
E.3.4.2
E.3.4.3
E.3.5
E.3.5.0
E.3.5.1
E.3.5.2
E.3.5.3
E.3.6
E.3.6.0
E.3.6.1
E.3.6.2
E.3.6.3
E.3.6.3.0
E.3.6.3.1
E.3.6.3.2
E.3.6.3.3
E.3.6.3.4
E.3.6.3.5
E.3.6.4
E.3.6.5
E.3.6.5.1
E.3.6.5.2
E.3.6.6
E.3.6.7
E.3.6.7.0
E.3.6.7.1
E.3.6.7.2
E.3.7
E.3.7.0
E.3.7.1
E.3.7.2
E.3.7.3
E.3.7.3.0
E.3.7.3.1
E.3.7.3.2
E.3.7.3.3
E.3.7.3.4
E.3.7.3.5
E.3.7.4
E.3.7.5
E.3.7.5.1
E.3.7.5.2
E.3.7.6
E.3.7.7
E.3.7.8
E.3.8
E.3.8.0
E.3.8.1
E.3.8.2
E.3.8.3
E.3.8.4
E.3.8.5
E.3.8.5.1
E.3.8.5.2
E.3.8.6

General aspects ................................................................................................................................ 117
Pilot structure ................................................................................................................................... 118
Modified VL-SNR-frame ................................................................................................................ 118
Format Specification 1: DVB-S2 legacy ............................................................................................... 119
Format Specification 2: Bundled PLFRAME (64 800 payload Size) with SF-Pilots ............................ 119
General aspects ................................................................................................................................ 119
Bundled PLFRAME (64 800 payload) Definition ........................................................................... 120
PLHEADER Specification for Bundled PLFRAMEs (64 800 payload) ......................................... 121
SF-Pilot Structure ............................................................................................................................ 123
Format Specification 3: Bundled PLFRAME (16 200 Payload Size) with SF-Pilots............................ 124
General aspects ................................................................................................................................ 124
Bundled PLFRAME Definition ....................................................................................................... 125
PLHEADER Specification for Short Bundled PLFRAME.............................................................. 126
SF-Pilot Structure ............................................................................................................................ 127
Format Specification 4: Flexible Format with VL-SNR PLH tracking ................................................. 129
General aspects ................................................................................................................................ 129
Super-Frame Header (SFH) ............................................................................................................. 130
SFH-Trailer (ST) ............................................................................................................................. 131
Physical Layer Header (PLH) .......................................................................................................... 131
General aspects .......................................................................................................................... 131
PLSCODE Definition................................................................................................................. 131
PLH Protection Levels ............................................................................................................... 132
Signalling of MOD/COD/SPREAD/SIZE ................................................................................. 133
Field for TSN ............................................................................................................................. 134
SOF Sequence ............................................................................................................................ 134
PLFRAME structure ........................................................................................................................ 134
Pilot structure ................................................................................................................................... 136
SF-Pilots ..................................................................................................................................... 136
Special VL-SNR Pilots .............................................................................................................. 136
Spreading and Signalling Rules ....................................................................................................... 136
Dummy PL Frame Definition .......................................................................................................... 137
General aspects .......................................................................................................................... 137
Dummy PL frames with deterministic content ........................................................................... 137
Dummy PL frames with arbitrary content .................................................................................. 138
Format Specification 5: Periodic Beam Hopping Format with VL-SNR and fragmentation Support... 138
General aspects ................................................................................................................................ 138
Super-Frame Header (SFH) ............................................................................................................. 140
SFH-Trailer (ST) ............................................................................................................................. 141
Physical Layer Header (PLH) .......................................................................................................... 141
General aspects .......................................................................................................................... 141
PLSCODE Definition................................................................................................................. 141
PLH Protection Levels ............................................................................................................... 141
Signalling of MOD/COD/SPREAD/SIZE and TYPE ................................................................ 141
Field for TSN ............................................................................................................................. 142
SOF Sequence ............................................................................................................................ 142
PLFRAME structure ........................................................................................................................ 142
Pilot structure ................................................................................................................................... 143
SF-Pilots ..................................................................................................................................... 143
Special VL-SNR Pilots .............................................................................................................. 143
Spreading and Signalling Rules ....................................................................................................... 143
Dummy PL Frame Definition .......................................................................................................... 143
Postamble Definition ....................................................................................................................... 143
Format Specification 6: Traffic Driven Beam Hopping Format with VL-SNR Support ....................... 144
General aspects ................................................................................................................................ 144
Super-Frame Header (SFH) ............................................................................................................. 146
Physical Layer Trailer (ST) ............................................................................................................. 146
Physical Layer Header (PLH) .......................................................................................................... 146
PLFRAME structure ........................................................................................................................ 146
Pilot structure ................................................................................................................................... 146
SF-Pilots ..................................................................................................................................... 146
Special VL-SNR Pilots .............................................................................................................. 146
Spreading and Signalling Rules ....................................................................................................... 147

ETSI

6

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.8.7
E.3.8.8
E.3.9
E.3.9.0
E.3.9.1
E.3.9.2
E.3.9.3
E.3.9.3.0
E.3.9.3.1
E.3.9.3.2
E.3.9.3.3
E.3.9.3.4
E.3.9.3.5
E.3.9.4
E.3.9.5
E.3.9.5.1
E.3.9.5.2
E.3.9.6
E.3.9.7
E.3.9.8
E.3.10

Dummy PL Frame Definition .......................................................................................................... 147
Postamble Definition ....................................................................................................................... 147
Format Specification 7: Simplified Traffic Driven Beam Hopping Format without VL-SNR Support 147
General aspects ................................................................................................................................ 147
Superframe Header (SFH) ............................................................................................................... 148
SFH-Trailer (ST) ............................................................................................................................. 148
Physical Layer Header (PLH) .......................................................................................................... 148
General aspects .......................................................................................................................... 148
PLSCODE Definition................................................................................................................. 148
PLH Protection Levels ............................................................................................................... 149
Signalling of MOD/COD/SPREAD/SIZE and TYPE ................................................................ 149
Field for TSN ............................................................................................................................. 149
SOF Sequence ............................................................................................................................ 149
PLFRAME structure ........................................................................................................................ 149
SF-Pilot structure ............................................................................................................................. 149
SF-Pilots ..................................................................................................................................... 149
Special VL-SNR Pilots .............................................................................................................. 149
Spreading and Signalling Rules ....................................................................................................... 149
Dummy PL Frame Definition .......................................................................................................... 149
Postamble Definition ....................................................................................................................... 149
Format Specifications 8 - 15: Reserved ................................................................................................. 149

E.4  Signalling of additional reception quality parameters via return channel (normative for

Interference Management at the Gateway)................................................................................... 150

Annex F:

Void .............................................................................................................................. 152

Annex G:

Void .............................................................................................................................. 153

Examples of possible use of the System .............................................. 154
Annex H (informative):
H.0  General aspects ............................................................................................................................. 154
H.1  Void .............................................................................................................................................. 154
H.2  Void .............................................................................................................................................. 154
H.3  Void .............................................................................................................................................. 154
H.4  Void .............................................................................................................................................. 154
H.5  Void .............................................................................................................................................. 154
H.6  Void .............................................................................................................................................. 154
H.7  Satellite transponder models for simulations ............................................................................... 154
H.8  Phase noise masks for simulations ............................................................................................... 156

ACM ...................................................................................................... 158
Annex I (normative):
I.1  ACM Command ........................................................................................................................... 158
I.2  Dummy Synchronization Scheme (optional) ............................................................................... 159
General aspects ...................................................................................................................................... 159
I.2.0
Dummy Synchronization Frame structure ............................................................................................. 159
I.2.1
General aspects ................................................................................................................................ 159
I.2.1.0
PLH* description ............................................................................................................................. 160
I.2.1.1
Known Symbols ............................................................................................................................... 160
I.2.1.2
Known correlation structure ............................................................................................................ 160
I.2.1.3
Scrambling ............................................................................................................................................ 161
I.2.2

Annex J:

Void .............................................................................................................................. 162

Annex K:

For future use .............................................................................................................. 163

Annex L:

For future use .............................................................................................................. 164

ETSI

7

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex M (normative):

Transmission format for wideband satellite transponders using time-
slicing (optional) ................................................................................... 165
History .................................................................................................................................................... 166

ETSI

8

ETSI EN 302 307-2 V1.4.1 (2024-08)

Intellectual Property Rights

Essential patents

IPRs essential or potentially essential to normative deliverables may have been declared to ETSI. The declarations
pertaining to these essential IPRs, if any, are publicly available for ETSI members and non-members, and can be
found in ETSI SR 000 314: "Intellectual Property Rights (IPRs); Essential, or potentially Essential, IPRs notified to
ETSI in respect of ETSI standards", which is available from the ETSI Secretariat. Latest updates are available on the
ETSI Web server (https://ipr.etsi.org/).

Pursuant to the ETSI Directives including the ETSI IPR Policy, no investigation regarding the essentiality of IPRs,
including IPR searches, has been carried out by ETSI. No guarantee can be given as to the existence of other IPRs not
referenced in ETSI SR 000 314 (or the updates on the ETSI Web server) which are, or may be, or may become,
essential to the present document.

Trademarks

The present document may include trademarks and/or tradenames which are asserted and/or registered by their owners.
ETSI claims no ownership of these except for any which are indicated as being the property of ETSI, and conveys no
right to use or reproduce any trademark and/or tradename. Mention of those trademarks in the present document does
not constitute an endorsement by ETSI of products, services or organizations associated with those trademarks.

DECT™, PLUGTESTS™, UMTS™ and the ETSI logo are trademarks of ETSI registered for the benefit of its
Members. 3GPP™ and LTE™ are trademarks of ETSI registered for the benefit of its Members and of the 3GPP
Organizational Partners. oneM2M™ logo is a trademark of ETSI registered for the benefit of its Members and of the
oneM2M Partners. GSM® and the GSM logo are trademarks registered and owned by the GSM Association.

Foreword

This European Standard (EN) has been produced by Joint Technical Committee (JTC) Broadcast of the European
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

The present document is part 2 of a multi-part deliverable covering the optional extensions of the DVB-S2 system,
denoted "DVB-S2X", as identified below:

Part 1:

"DVB-S2";

Part 2:  "DVB-S2 Extensions (DVB-S2X)".

ETSI

9

ETSI EN 302 307-2 V1.4.1 (2024-08)

National transposition dates

Date of adoption of this EN:

Date of latest announcement of this EN (doa):

Date of latest publication of new National Standard
or endorsement of this EN (dop/e):

Date of withdrawal of any conflicting National Standard (dow):

15 August 2024

30 November 2024

31 May 2025

31 May 2025

Modal verbs terminology

In the present document "shall", "shall not", "should", "should not", "may", "need not", "will", "will not", "can" and
"cannot" are to be interpreted as described in clause 3.2 of the ETSI Drafting Rules (Verbal forms for the expression of
provisions).

"must" and "must not" are NOT allowed in ETSI deliverables except when used in direct citation.

Introduction

The optional extensions of the S2 system have been approved in 2014 and are identified by the S2X denomination.
Such extensions are non-backwards-compatible with ETSI EN 302 307 [4], are optional for the implementation of new
receivers under ETSI EN 302 307-1 [3], but are normative for the implementation of receivers under the present
document: mapping of specific S2X building blocks to application areas is specified in Table 1. For every S2X
application area, as defined in Table 1, the configurations for the corresponding S2 application area, as defined in ETSI
EN 302 307-1 [3], Table 1, will be implemented. In case of conflicts the definition of the S2X application area applies.

The present document targets the core application areas of S2 (Digital Video Broadcasting, forward link for interactive
services using ACM, Digital Satellite News Gathering and professional digital links such as video point-to-point or
Internet trunking links), and new application areas requiring very-low carrier-to-noise and carrier-to-interference
operation (VL-SNR).

In particular for DTH, a possible use case is the launch of UHDTV-1 (e.g. 4k) television services in Ku-/Ka-band that
will adopt HEVC encoding. In this context it may be desirable to eventually use fragments of smaller blocks of capacity
on two or three DTH transponders and bond them into one logical stream. This permits to maximize capacity
exploitation by avoiding the presence of spare capacity in individual transponders and/or to take maximum advantage of
statistical multiplexing.

The S2X system offers the ability to operate with very-low carrier-to-noise and carrier-to-interference ratios (SNR
down to -10 dB), to serve markets such as airborne (business jets), maritime, civil aviation internet access, VSAT
terminals at higher frequency ranges or in tropical zones, small portable terminals for journalists and other
professionals. Furthermore, the S2X system provides transmission modes offering significantly higher capacity and
efficiency to serve professional links characterized by very-high carrier-to-noise and carrier-to-interference ratios
conditions.

The present document reuses the S2 system architecture, while adding finer MODCOD steps, sharper roll-off filtering,
technical means for bonding of multiple transponders and additional signalling capacity by means of an optional
periodic super-frame structure, extended PLHEADER signalling schemes and the support of GSE-Lite signals.

The present document maintains the same clause numbering as ETSI EN 302 307-1 [3], in order to facilitate
cross-reference.

ETSI

10

ETSI EN 302 307-2 V1.4.1 (2024-08)

1

Scope

The present document specifies the optional extensions of the S2 system, identified by the S2X denomination. The
present document also includes amendments to the standard to enable beam hopping operation.

2

References

2.1

Normative references

References are either specific (identified by date of publication and/or edition number or version number) or
non-specific. For specific references, only the cited version applies. For non-specific references, the latest version of the
referenced document (including any amendments) applies.

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

[10]

[11]

ETSI TS 101 545-1 (V1.1.1): "Digital Video Broadcasting (DVB); Second Generation DVB
Interactive Satellite System (DVB-RCS2); Part 1: Overview and System Level specification".

ETSI TS 102 606-1 (V1.2.1): "Digital Video Broadcasting (DVB); Generic Stream Encapsulation
(GSE); Part 1: Protocol".

ETSI EN 302 307-1: "Digital Video Broadcasting (DVB); Second generation framing structure,
channel coding and modulation systems for Broadcasting, Interactive Services, News Gathering
and other broadband satellite applications; Part 1: DVB-S2".

ETSI EN 302 307 (V1.1.1): "Digital Video Broadcasting (DVB); Second generation framing
structure, channel coding and modulation systems for Broadcasting, Interactive Services, News
Gathering and other broadband satellite applications".

ETSI EN 300 468: "Digital Video Broadcasting (DVB); Specification for Service Information (SI)
in DVB systems".

ETSI TS 102 606-2: "Digital Video Broadcasting (DVB); Generic Stream Encapsulation (GSE);
Part 2: Logical Link Control (LLC)".

ETSI ETS 300 801: "Digital Video Broadcasting (DVB); Interaction channel through Public
Switched Telecommunications Network (PSTN)/Integrated Services Digital Networks (ISDN)".

ETSI EN 301 195: "Digital Video Broadcasting (DVB); Interaction channel through the Global
System for Mobile communications (GSM)".

ETSI ES 200 800: "Digital Video Broadcasting (DVB); DVB interaction channel for Cable TV
distribution systems (CATV)".

ETSI ETS 300 802: "Digital Video Broadcasting (DVB); Network-independent protocols for DVB
interactive services".

ETSI EN 301 790: "Digital Video Broadcasting (DVB); Interaction channel for satellite
distribution systems".

ETSI

11

ETSI EN 302 307-2 V1.4.1 (2024-08)

2.2

Informative references

References are either specific (identified by date of publication and/or edition number or version number) or
non-specific. For specific references, only the cited version applies. For non-specific references, the latest version of the
referenced document (including any amendments) applies.

NOTE:  While any hyperlinks included in this clause were valid at the time of publication, ETSI cannot guarantee

their long term validity.

The following referenced documents are not necessary for the application of the present document but they assist the
user with regard to a particular subject area.

Not applicable.

3

Definition of terms, symbols and abbreviations

Terms

3.1

Void.

3.2

Symbols

For the purposes of the present document, the symbols given in ETSI EN 302 307-1 [3] and the following apply:

dSF
PSF
HST
HSOSF
RS

SF-pilot distances
SF-pilot field length
SFH-Trailer (ST) Matrix
Start Of SuperFrame Matrix
Symbol rate corresponding to the bilateral Nyquist bandwidth of the modulated signal

3.3

Abbreviations

For the purposes of the present document, the abbreviations given in ETSI EN 302 307-1 [3] and the following apply:

128APSK
256APSK
64APSK
BBF
BH
BHTC
BHTP
BPSK
CNTR
CU
DT
EHF
EXOR
FER
GSE
GSE-HEM
GSE-LLC
HEVC
PLH
PLI
RFU
SF

128-ary Amplitude and Phase Shift Keying
256-ary Amplitude and Phase Shift Keying
64-ary Amplitude and Phase Shift Keying
Base Band Frame
Beam Hopping
Beam Hopping Transmission Channel
Beam Hopping Time Plan
Binary Phase Shift Keying
Counter
Capacity Unit
Dwell Time
Extended Header Field
Exclusive-OR (logical operator/function)
Frame Error Rate
Generic Stream Encapsulation
Generic Stream Encapsulation - High Efficiency Mode
Generic Stream Encapsulation - Logical Link Control
High Efficiency Video Coding
Physical Layer Header
Protection Level Indication
Reserved for Future Use
Super-Frame

ETSI

12

ETSI EN 302 307-2 V1.4.1 (2024-08)

SFFI
SFH
SFL
SOSF
ST
UHDTV
VL-SNR
VSAT
WH

Super-Frame Format Indicator
Super-Frame Header
Super Frame Length
Start Of Super-Frame
Super-Frame Header Trailer
Ultra High Definition TeleVision
Very Low - Signal to Noise Ratio
Very Small Aperture Terminal
Walsh-Hadamard

4

Transmission system description

4.0

General aspects

See ETSI EN 302 307-1 [3], clause 4.

4.1

System definition

See ETSI EN 302 307-1 [3], clause 4.1.

4.2

System architecture

See ETSI EN 302 307-1 [3], clause 4.2.

The present document reuses the S2 system architecture as described in ETSI EN 302 307-1 [3], Figure 1, while adding
finer MODCOD steps, sharper roll-off filtering, technical means allowing time-slicing of wide-band signals (for a
reduced processing speed in the receiver), technical means for bonding of multiple transponders, among other
technologies.

Additional signalling capacity is provided:

•

•

•

•

•

an optional periodic super-frame structure with signalling of the format of the super-frame content and further
benefits like simplifying synch recovery at VL-SNR and allowing periodic pilot structures and PL-Scramblers;

an extended PLHEADER signalling scheme to support the additional MODCODs;

an extended PLHEADER signalling scheme to support Mobile Frames (VL-SNR);

a high-efficiency BBFRAME mode (GSE-HEM), similar to the T2 and C2 systems, to transport
GSE/GSE-Lite packets;

signalling of streams which are GSE-Lite compliant.

Annex E includes optional additional formats to enable operation of beam -hopping. The specified waveforms provide
additional signalling and framing options that support both periodic, pre-scheduled beam hopping operation, as well as
random, traffic driven illumination policy, at signal to noise ratios ranging from -10 dB and above.

4.3

System configurations

See ETSI EN 302 307-1 [3], clause 4.3.

Table 1 associates the S2X system elements to the applications areas. All elements in Table 1 are optional in
transmitting and receiving equipment complying with the S2 specification. At least "Normative" subsystems and
functionalities shall be implemented in the transmitting and receiving equipment to comply with the present document
for a specific application area.

ETSI

13

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 1: S2X System configurations and application areas

System configurations

Broadcast
services

Interactive
services

DSNG

Professional
services

VL-SNR

FECFRAME (normal)
(see MODCODs below)
QPSK

8PSK

8APSK-L (note 7)
16APSK

16APSK-L (note 7)
32APSK

32APSK-L (note 7)
64APSK
64APSK-L (note 7)
128APSK
256APSK
256APSK-L (note 7)

FECFRAME (short)
(see MODCODs below)
QPSK

8PSK

16APSK

64 800 (bits)

1/4,1/3, 2/5 (S2-MODCODs)
1/2, 3/5, 2/3, 3/4, 4/5, 5/6, 8/9,
9/10 (S2-MODCODs)
13/45
9/20; 11/20
3/5, 2/3, 3/4, 5/6, 8/9, 9/10
(S2-MODCODs)
23/36; 25/36; 13/18
5/9;26/45
2/3, 3/4, 4/5, 5/6, 8/9, 9/10
(S2-MODCODs)
26/45; 3/5; 28/45; 23/36; 25/36;
13/18; 7/9; 77/90
5/9; 8/15; 1/2; 3/5; 2/3
3/4, 4/5, 5/6, 8/9, 9/10
(S2-MODCODs)
32/45; 11/15; 7/9
2/3
11/15; 7/9; 4/5; 5/6
32/45
3/4; 7/9
32/45; 3/4
29/45; 2/3; 31/45; 11/15

16 200 (bits)

1/4,1/3, 2/5
(S2-MODCODs)
1/2, 3/5, 2/3, 3/4, 4/5, 5/6, 8/9
(S2-MODCODs)
11/45; 4/15; 14/45; 7/15 8/15;
32/45
3/5, 2/3, 3/4, 5/6, 8/9
(S2-MODCODs)
7/15; 8/15; 26/45; 32/45
2/3, 3/4, 4/5, 5/6, 8/9
(S2-MODCODs)
7/15; 8/15; 26/45; 3/5; 32/45

N
N

N
N

N

N
N
N

N

N
N

N
N
O
O
NA
NA
NA

NA

NA

NA

NA

NA
NA

NA

ETSI

N
N

N
N

N

N
N
N

N

N
N

N
N
N
N
O
O
O

N

N

N

N

N
N

N

N
N

N
N

N

N
N
N

N

N
N

N
N
N
N
O
O
O

O

O

O

O

O
O

O

N
N

N
N

N

N
N
N

N

N
N

N
N
N
N
N
N
N

N

N

N

N

N
N

N

N
N

N
N
N

N
N
N

N

N
N

N
N
O
O
NA
NA
NA

N

N

N

N

N
N

N

14

ETSI EN 302 307-2 V1.4.1 (2024-08)

32APSK

System configurations

3/4, 4/5, 5/6, 8/9
(S2-MODCODs)
2/3; 32/45

2/9 (normal)
1/5; 4/15; 1/3 (short)
1/5; 11/45; 1/3 (medium)
1/5; 11/45 (short)

8-bits

8+8 bits (time slicing)

For GSE/GSE-Lite
(note 6)

VL-SNR Header
(see MODCODs below) (note 1)
QPSK
BPSK

BPSK-S
Spreading Factor 2

Fixed Size Super-frame (notes 8 and 11)

Part 2 PLHEADER
(note 5)
Extended PLHEADER
For Wide-band mode (note 5)
GSE-High Efficiency Mode

Roll-off 0,15; 0,10 and 0,05
Channel bonding (note 2)

VCM
(note 4)
ACM
Beam Hopping Periodic BH, VLSNR
(note 8) (Superframe Format 5) (note 10)
Traffic driven BH VLSNR (note 8)
(Superframe Format 6)

Broadcast
services
NA

Interactive
services
N

NA

O

NA
NA

NA

NA

N

O

N

N
N
(note 3)
N

NA
O

O

N

O

O
O

O

O

N

O

N

N
NA

N

N
O

O

DSNG

O

O

O

O
O

O

O

N

NA

N

N
NA

N

O
O

O

Professional
services
N

N

NA

NA
NA

NA

O

N

O

N

N
O

N

O
O

O

VL-SNR

N

N

N

N
N

N

O/NA
(note 9)
N

O

N

N
NA

N

N
O

O

ETSI

15

ETSI EN 302 307-2 V1.4.1 (2024-08)

System configurations

Broadcast
services
O

Interactive
services
O

DSNG

VL-SNR

Professional
services
O

Traffic driven BH, no VL-SNR (note 8)
(Superframe Format 7)
N = normative, O = optional, NA = not applicable.
NOTE 1:  Ability to skip VL-SNR frames: Normative.
NOTE 2:  Requires Input Stream Synchronizer, Null-Packet Deletion and Dummy Frame insertion.
NOTE 3:  Normative for broadcast services in case of optional multiple tuner receivers.
NOTE 4:  Any S2X receiver shall be able to recognize the whole set of MODCODS within the PLHeader and skip the XFECFrame if the MODCOD is not

O

NA

supported.

NOTE 5:  The present document, PLHEADER and Extended PLHEADER for wideband transponders (ETSI EN 302 307-1 [3] or ETSI EN 302 307-2 (the

present document), Annex M) cannot coexist in the same carrier but either can coexist with the VL-SNR header.

NOTE 6:  GSE is optional while support for GSE-Lite in GSE-HEM is normative across all the services.
NOTE 7:  xxx-L= MODCODs optimized for quasi-linear channels.
NOTE 8:  Each of the Annex E formats are individually optional.
NOTE 9:  Not all Annex E Super-Frame Formats support VL-SNR. They are different from the VL-SNR XFECFRAMEs in clause 5.5.2.
NOTE 10: Format 5 can also be used for continuous transmission scenarios.
NOTE 11: Fixed size Superframes refer to Annex E Formats 0, 1, 2, 3 and 4.

ETSI

16

ETSI EN 302 307-2 V1.4.1 (2024-08)

Within the present document, a number of configurations and mechanisms are defined as "Optional". Configurations
and mechanisms explicitly indicated as "optional" within the present document, for a given application area, need not be
implemented in the equipment to comply with the present document. Nevertheless, when an "optional" mode or
mechanism is implemented, it shall comply with the specification as given in the present document.

5

Subsystems specifications

5.1

Mode adaptation

5.1.0  General aspects

See ETSI EN 302 307-1 [3], clause 5.1.

According to Figure 3, the input sequence(s) is (are):

•

•

Single or multiple Transport Streams (TS).

Single or multiple Generic Streams (packetized, continuous or High-Efficiency Mode (HEM) packetized).

The output sequence is a BBHEADER (80 bits) followed by a DATA FIELD.

5.1.1

Input Interfaces

See ETSI EN 302 307-1 [3], clause 5.1.1.

An efficient input interface has been introduced as GSE-HEM. For details of GSE-HEM, see clause 5.1.7.

5.1.2

Input stream synchronizer (optional, not relevant for single TS - BS)

See ETSI EN 302 307-1 [3], clause 5.1.2.

5.1.3

Null-Packet Deletion (ACM and Transport Stream only)

See ETSI EN 302 307-1 [3], clause 5.1.3.

5.1.4

CRC-8 encoder (for packetized streams only)

See ETSI EN 302 307-1 [3], clause 5.1.4.

5.1.5  Merger/Slicer

See ETSI EN 302 307-1 [3], clause 5.1.5.

5.1.6

Base-Band Header insertion

See ETSI EN 302 307-1 [3], clause 5.1.6.

First byte (MATYPE-1):

•  TS/GS field (2 bits): Transport Stream Input, Generic Stream Input (packetized or continuous) or GSE-HEM.

•  SIS/MIS field (1 bit): Single Input Stream or Multiple Input Stream.

•  CCM/ACM field (1 bit): Constant Coding and Modulation or Adaptive Coding and Modulation (VCM is

signalled as ACM).

ETSI

•

ISSYI (1 bit), (Input Stream Synchronization Indicator): If ISSYI = 1 = active, the ISSY field (see Annex D) is
inserted after UPs or in the baseband header in GSE-HEM.

17

ETSI EN 302 307-2 V1.4.1 (2024-08)

•  For TS input mode:

-

NPD (1 bit): Null-packet deletion active/not active.

•  For GSE/Generic Continuous/Generic Packetized modes:

-

GSE-Lite (1 bit): GSE stream is GSE-Lite compliant/non-compliant.

•  RO (2 bits): Transmission Roll-off factor (α). Three additional roll-off factors shall be available, 0,15; 0,10

and 0,05. Signalling shall be according to the following rule (Table 2):

-

-

If RO bits are signalled consistently from BBHEADER to BBHEADER as either 00, 01, 10 the
backward compatible definition (High roll-off range) applies:


00 = 0,35.





01 = 0,25.

10 = 0,20.

If RO bits are signalled from BBHEADER to BBHEADER in an alternating way with 11 then their
interpretation shall be Low roll-off range:


00 = 0,15.





01 = 0,10.

10 = 0,05.

It shall be ensured that in a Multiple Input Stream configuration (SIS/MIS field = 0) alternation is
unambiguously evident over all Input Streams (for every ISI) and MODCOD combinations, such that any
receiver will receive regular alternation. Any receiver, once locked will switch to low roll-off range on first
detection of '11'.

Table 2 (see ETSI EN 302 307-1 [3], Table 3): MATYPE-1 field mapping

TS/GS

SIS/MIS

CCM/ACM

ISSYI

NPD/GSE-Lite

RO

11 = Transport
00 = Generic Packetized
01 = Generic continuous
10 = GSE-HEM
NOTE:  GSE-Lite signals are defined in Annex D of ETSI TS 102 606-1 [2].

1 = active
0 = not-active

1 = single
0 = multiple

1 = CCM
0 = ACM

1 = active
0 = not-active

No
Alternation
with 11 =
high roll-off
range
00 = 0,35
01 = 0,25
10 = 0,20

Alternation
with 11 =
low roll-off
range
00 = 0,15
01 = 0,10
10 = 0,05

5.1.7  GSE High Efficiency Mode (GSE-HEM)

GSE variable-length or constant length UPs may be transmitted in GSE-HEM. In GSE-HEM, slicing of GSE packets is
performed and SYNCD shall always be computed. The receiver may derive the length of the UPs from the packet
header, therefore UPL transmission in BBHEADER is not performed. In the case where ISSY is active, UPs shall not
be sliced when there is a BBFRAME from a different stream following, splitting is only possible with the immediately
following BBFRAME. The optional ISSY field is transmitted in the BBHEADER.

The Mode Adaptation unit shall perform the following sequence of operations (see Figure 1):

•

Optional input stream synchronization (see ETSI EN 302 307-1 [3], clause D.2) relevant to the first
transmitted UP which starts in the data field; ISSY field inserted in the UPL and SYNC fields of the
BBHEADER.

ETSI

18

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

•

•

•

•

Null-packet Deletion and CRC-8 at UP level shall not be computed nor inserted.

SYNCD computation (pointing at the first bit of the first transmitted UP which starts in the Data Field) and
storage in BBHEADER. The transmitted UP corresponds exactly to the original UP itself. Hence SYNCD
points to the first bit of the original UP.

CRC8_MODE computation. This is the EXOR of the CRC-8 (1-byte) field with the MODE (1-byte) field.
CRC-8 is the error detection code applied to the first 9 bytes of the BBHEADER. MODE (8 bits) shall be 1_D
for GSE-HEM.

UPL not computed nor transmitted.

GSE-Lite compliance of the stream shall be signalled in the 6th bit of the MATYPE-1 field. GSE-Lite=1
means a GSE-Lite compliant signal is transmitted. GSE-Lite=0 means that the transmitted GSE stream may
not meet the definition of a GSE-Lite signal.

G SE

      UP

UPL (in G SE Headers)

Tim e

      UP

UP

              UP

UP

80 bits

SYN C D

User Packet

D FL

BBHEADER

D AT A FIELD

M A T YP E
(2 bytes)

IS S Y
(2 M S B)

D FL
(2 bytes)

IS S Y
(1 LS B )

S YNC D
(2 bytes)

CR C-8
MO DE  (1 byte)

Optional

Figure 1: Stream format at the output of the MODE ADAPTER, High Efficiency Mode for GSE
(no CRC-8 computed for UPs, optional single ISSY inserted
in the BBHEADER, UPL not transmitted)

5.1.8

Channel bonding for multi-tuner (L) receivers

5.1.8.1

Introduction to channel bonding

The present document provides tools to implement "channel bonding", where a single input stream is carried in parallel
over L transponders. The maximum number of bonded transponders shall be 3 (L ≤ 3).

Channel bonding allows for example to avoid un-used capacity in a transponder in case of Constant Bit-Rate (CBR)
video programmes, and /or to maximize the statistical multiplexing gain in case of Variable Bit-Rate (VBR) video
programmes.

The bonded channels shall lie in the same frequency band. Further, channel bonding shall use CCM only, and shall not
be combined with wideband tuners (according to Annex M of ETSI EN 302 307-1 [3] and Annex M of the present
document).

In the following clauses, channel bonding for TS transmission (clause 5.1.8.2) and for GSE (clause 5.1.8.3) will be
described in more detail.

ETSI

19

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.1.8.2

Channel bonding for TS transmission

Channel bonding for TS transmission allows a single "big-Transport-Stream" to be carried in parallel over L
transponders (L ≤ 3). This requires that the receivers are equipped with L tuners/S2X decoders, receiving in parallel the
L "partial" Transport Streams from the L transponders, and reconstructing the original "big-Transport-Stream". The L
S2X modulators are allowed to adopt the same symbol-rate and MODCOD or different ones.

The number of bonded transponders and their carrier frequencies are signalled in the SI tables according to ETSI
EN 300 468 [5]. These SI tables shall be transmitted in parallel over each of the bonded transponders. This allows an
initial signal scan with a single tuner to extract SI tables. The principle of the S2X transmitting side shall be according
to Figure 2, where the L S2X modulators use the same modulo 222 ISSY counter, clocked by the symbol-rate of a
master channel (in Figure 2, modulator number 1 as example), to implement Input-Stream Synchronization (ISSY, see
ETSI EN 302 307-1 [3], clause D.2). The correspondence between the RF channel and master channel shall be signalled
to the receivers via the SI. Null-Packet deletion is implemented in all modulators according to ETSI EN 302 307-1 [3],
clause D.3.

The input "big-TS" shall be split at TS-packet level over L branches, as follows:

•

•

For PIDs ∉ {SI tables}, when a TS packet is routed into a branch, corresponding Null Packets shall be
generated on the other output branches.

For PIDs ∈ {SI tables}, the packet shall be copied in all the output branches.

Each input packet with PID ∉ {SI tables} shall be routed into a branch such that the interval between two useful packets
with PIDs ∉ {SI tables} (in terms of TS packets) which are separated by Null Packets, not including packets with PIDs
∈ {SI tables}, generated in the SPLIT block, is kept to a minimum and as uniform as possible.

The useful packet intervals shall be according to the ratio of the total bitrate of the bonded channels to the TS rate of
each channel.

For example for L = 2 channels, this can be fulfilled if the useful packet interval of transponder k takes on only two
different values:

floor(total TS rate/TS rate of transponder k) and/or ceil(total TS rate/TS rate of transponder k),

in which floor(x) and ceil(x) denote the flooring and ceiling operation, respectively. The useful packet interval is
defined as the number of Null Packets, not including packets with PIDs ∈ {SI tables}, inserted into two useful packets
in the SPLIT block plus 1. For example, in Figure 2 the useful packets 1 and 3 are separated by one Null Packet in
transponder 1, resulting in a useful packet interval of 2.

The TS rate of each transponder k = 1, 2…, L is the rate used for transferring packets with PIDs ∉{SI tables} in
channel bonding on this transponder. This corresponds to the total TS rate of the transponder minus the data rate
occupied by PIDs ∈{SI tables}. The total TS rate in above equations is the sum of such TS rates of all transponders.

Each S2X modulator shall activate Input Stream Synchronization by setting the suitable ISSY field.

Transport Stream rate-adapters (i.e. adding or deleting Null-Packets and adjusting the MPEG time-stamps) shall not be
inserted after the SPLIT.

NOTE:  Rate-adapters may be inserted before the SPLIT if required.

Clause D.1 shows rules for implementation of channel bonding for TS transmissions.

ETSI

20

ETSI EN 302 307-2 V1.4.1 (2024-08)

“big Input-TS”
 TS packets

4

3

2

1

M  …

4

3

NP

1

1

SPLIT

2

L

Buffers

NP

NP

2

NP

Ch#1

Input-
Stream

Synch

FEC/

MOD

Symbol

Null

Packet
Deletion
ISSY

Counter

S2X modulator (1)

Input-
Stream

Synch

Null

Packet
Deletion

FEC/

MOD

Ch#L

Figure 2: Principle of the transmitting modulators configurations for channel bonding

5.1.8.3

Channel bonding for GSE transmission

5.1.8.3.0

General aspects

Generic Stream Encapsulation (GSE) (ETSI TS 102 606-1 [2]) is an extremely flexible method to transmit any kind of
data, including popular formats such as IP packets or TS packets where the data can be of fixed or variable length. GSE
can be used for bonded channels to support a higher data rate than can be carried in a single RF channel. A maximum of
L channels (L ≤ 3) is supported. The number of bonded transponders and associated information is signalled in the
GSE-LLC tables according to ETSI TS 102 606-2 [6]. These GSE-LLC tables shall be transmitted in parallel over each
of the bonded transponders. To ensure maximum efficiency in S2X, it is recommended to use GSE-HEM (see
clause 5.1.7). The following describes the use of channel boding in GSE-HEM.

Channel bonding for GSE transmission is similar to the TS method of bonding described in clause 5.1.8.2, using the
ISCR timing data in the ISSY field to allow the receiver to align packets from different RF channels (see ETSI
EN 302 307-1 [3], Annex D for ISSY details). However ISSY is not added per UP, but per baseband frame
(BBFRAME). ISSY shall always be used for bonded GSE channels. In the ISSY field, ISCR shall be transmitted every
BBFRAME. BUFS and BUFSTAT shall not be transmitted.

At the modulator, input UPs (GSE packets) are continuously added to the Data Field of a single BBFRAME until it is
complete. Appropriate ISSY information is added to the baseband frame header (BBHEADER) of each BBFRAME.
ISSY information refers to the first transmitted UP which starts in the Data Field. UPs shall be transparently sliced
between BBFRAMEs on different RF channels as necessary - it is not required to slice UPs on BBFRAMEs using the
same RF channel. The order of input UPs shall be maintained in the bonding process. Each BBFRAME is constructed
with a length that is derived according to the modulation and coding parameters for that RF channel. Each RF channel
may have different modulation and coding parameters. In order to reduce buffering requirements, BBFRAMEs shall be
created for each RF channel according to the ratio of the bitrate of each RF channel. For example if the bitrates of two
bonded RF channels are equal, BBFRAMEs for each RF channel shall occur in alternating fashion.

An example of the transmission of bonded GSE channels is shown in Figure 3.

ETSI

21

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 3: Example of GSE channel bonding transmission

At the receiver side, each GSE bonded RF channel is demodulated according to the modulation and coding parameters
for that RF channel. An example diagram is shown in Figure 4.

The output from each demodulator is then combined at the Merger using the ISSY information contained in the
BBHEADER of each BBFRAME. The ISSY information provides the timing information to recover the order of the
BBRAMES from different demodulators. Since ISSY information applies to each BBFRAME, and the packet order of
UPs within each BBFRAME is maintained, the overall order of UPs is maintained at the Merger output. Split UPs are
reconstructed in the Merger.

In comparison to the TS method, the output bitrate of each demodulator is no greater than the bitrate of the channel,
which can significantly reduce the processing burden at the Merger. Furthermore, since ISSY information need only be
processed per BBFRAME, the merging operation processing burden is also reduced. A maximum tolerance of one
BBFRAME of delay shall be allowed between the different receivers.

Figure 4: Example of GSE channel bonding at the receiver

After merging, additional processing such as filtering of GSE packets, output of IP or TS packets rather than GSE
packets, and so on may be undertaken at the receiver as necessary.

The following text refers to GSE use in channel bonding for the mode TS/GS=00 (Generic Packetized) and TS/GS=01
(Generic Continuous).

ETSI

22

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.1.8.3.1

Channel bonding for Generic Packetized streams

For Generic Packetized streams, ISSY shall be added on a per packet basis the same as for TS channel bonding. CRC-8
shall be added per packet, as described in ETSI EN 302 307-1 [3], clause 5.1.5. SYNCD shall be computed and point to
the first bit of the CRC-8 of the previous UP. Packets shall only be split on the same RF channel.

NOTE:  For channel bonding using Generic Packetized streams, only ISCR_SHORT is allowed. Therefore, the

use of this mode is not recommended since timing constraints may not allow correct alignment of
packets.

5.1.8.3.2

Channel bonding for Generic Continuous streams

For Generic Continuous streams using GSE, ISSY shall be added on a per packet basis the same as for TS channel
bonding. CRC-8 computation shall not be performed. SYNCD shall be computed and point to the first transmitted UP
in the Data Field. The UPL field may contain proprietary signalling, including information about channel bonding,
otherwise the UPL field shall be set to 0. GSE Packets shall only be split on the same RF channel.

NOTE:  For channel bonding using Generic Continuous streams, the use of ISCR_SHORT is not recommended

since timing constraints may not allow correct alignment of packets.

5.2

Stream Adaptation

5.2.0  General aspects

See ETSI EN 302 307-1 [3], clause 5.2.

5.2.1

Padding

(Kbch-DFL-80) bits shall be appended after the DATA FIELD. The resulting BBFRAME shall have a constant length of
Kbch bits. For Broadcast Service applications, DFL = Kbch -80, therefore no padding shall be applied.

NOTE:  The difference with ETSI EN 302 307-1 [3], clause 5.2.1 is that here the appended bits are not

mandatorily zero.

5.2.2

BB scrambling

See ETSI EN 302 307-1 [3], clause 5.2.2.

5.3

FEC Encoding

5.3.0  General aspects

See ETSI EN 302 307-1 [3], clause 5.3.

In addition to ETSI EN 302 307-1 [3], clause 5.3 FEC, new coding rates and modulation formats are available as
described in the current clause and in clause 5.4. For VL-SNR support an additional FECFRAMEs is defined with
nldpc = 32 400 bits covering only BPSK modulation, coding rates 1/5, 11/45, 1/3 and requiring puncturing and
shortening as defined in clause 5.5.2.6.

NOTE:  LDPC Code Identifier 1/5 for short FECFRAME nldpc = 16 200 refers to the LDPC code defined in

ETSI EN 302 307-1 [3], clause 5.3 and identified with the LDPC code identifier 1/4 for short
FECFRAME nldpc = 16 200.

ETSI

23

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 3: Void

Table 4 (see Table 5a of ETSI EN 302 307-1 [3]): Coding Parameters
(for normal FECFRAME nldpc = 64 800)

LDPC Code
Identifier

2/9
13/45
9/20
90/180
96/180
11/20
100/180
104/180 and
26/45
18/30
28/45
23/36
116/180
20/30
124/180
25/36
128/180
13/18
132/180 and
22/30
135/180
140/180 and 7/9
154/180

BCH uncoded
block Kbch
14 208
18 528
28 968
32 208
34 368
35 448
35 808
37 248

BCH coded block Nbch
LDPC uncoded block kldpc
14 400
18 720
29 160
32 400
34 560
35 640
36 000
37 440

38 688
40 128
41 208
41 568
43 008
44 448
44 808
45 888
46 608
47 328

48 408
50 208
55 248

38 880
40 320
41 400
41 760
43 200
44 640
45 000
46 080
46 800
47 520

48 600
50 400
55 440

NOTE:

VL-SNR puncturing and shortening is defined in clause 5.5.2.6.

BCH t-error
correction

12
12
12
12
12
12
12
12

12
12
12
12
12
12
12
12
12
12

12
12
12

LDPC coded block
nldpc
61 560 (note)
64 800
64 800
64 800
64 800
64 800
64 800
64 800

64 800
64 800
64 800
64 800
64 800
64 800
64 800
64 800
64 800
64 800

64 800
64 800
64 800

Table 5: Coding Parameters (for medium FECFRAME nldpc = 32 400)

LDPC Code
Identifier

1/5
11/45
1/3

BCH uncoded
block Kbch
5 660 (note)
7 740
10 620

BCH coded block Nbch
LDPC uncoded block kldpc
5 840 (note)
7 920
10 800

BCH t-error
correction

12
12
12

LDPC coded block
nldpc
30 780 (note)
30 780 (note)
30 780 (note)

NOTE:

VL-SNR puncturing and shortening is defined in clause 5.5.2.6.

Table 6 (see Table 5b of ETSI EN 302 307-1 [3]): Coding Parameters
(for short FECFRAME nldpc = 16 200)

LDPC Code
Identifier

11/45
4/15
14/45
7/15
8/15
26/45
32/45

BCH uncoded
block Kbch
3 792
4 152
4 872
7 392
8 472
9 192
11 352

BCH coded block Nbch
LDPC uncoded block kldpc
3 960
4 320
5 040
7 560
8 640
9 360
11 520

NOTE:

VL-SNR puncturing and shortening is defined in clause 5.5.2.6.

BCH t-error
correction

12
12
12
12
12
12
12

LDPC coded block
nldpc
15 390 (note)
14 976 (note)
16 200
16 200
16 200
16 200
16 200

The addresses of parity bit accumulators of the S2X additional codes are given in Annex B (for nldpc = 64 800 bits) and
Annex C (for nldpc = 16 200 bits for nldpc = 32 400 bits).

ETSI

24

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.3.1  Outer encoding (BCH)

See ETSI EN 302 307-1 [3], clause 5.3.1.

Table 7: BCH Polynomials for Medium FECFRAME nldpc = 32 400)

g1(x)
g2(x)
g3(x)
g4(x)
g5(x)
g6(x)
g7(x)
g8(x)
g9(x)
g10(x)
g11(x)
g12(x)

1+x2+x3+x5+x15
1+x+x4+x7+x10+x11+x15
1+x2+x4+x6+x8+x10+x12+x13+x15
1+x2+x3+x5+x6+x8+x10+x11+x15
1+x+x2+x4+x6+x7+x10+x12+x15
1+x4+x6+x7+x12+x13+x15
1+x2+x4+x5+x7+x11+x12+x14+x15
1+x2+x4+x6+x8+x9+x11+x14+x15
1+x+x2+x4+x5+x7+x9+x11+x12+x13+x15
1+x+x2+x3+x4+x7+x10+x11+x12+x13+x15
1+x+x2+x4+x9+x11+x15
1+x2+x4+x8+x10+x11+x13+x14+x15

5.3.2

Inner encoding (LDPC)

5.3.2.0

General aspects

See ETSI EN 302 307-1 [3], clause 5.3.2.

5.3.2.1

Inner coding for normal FECFRAME

See ETSI EN 302 307-1 [3], clause 5.3.2.1.

Table 8a (see Table 7a of ETSI EN 302 307-1 [3]): q values for Normal FECFRAME

LDPC Code Identifier
2/9
13/45
9/20
90/180
96/180
11/20
100/180
104/180 and 26/45
18/30
28/45
23/36
116/180
20/30
124/180
25/36
128/180
13/18
132/180 and 22/30
135/180
140/180 and 7/9
154/180

q
140
128
99
90
84
81
80
76
72
68
65
64
60
56
55
52
50
48
45
40
26

ETSI

25

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.3.2.2

Inner coding for short and medium FECFRAME

See ETSI EN 302 307-1 [3], clause 5.3.2.2.

Table 8b (see Table 7b of ETSI EN 302 307-1 [3]): q values for Short FECFRAME

LDPC Code Identifier
11/45
4/15
14/45
7/15
8/15
26/45
32/45

q
34
33
31
24
21
19
13

Table 8c: q values for Medium FECFRAME

LDPC Code Identifier
1/5
11/45
1/3

q
72
68
60

For 128APSK padding is introduced to have an integer number of constellation points and slots in a FECFRAME.
6 zeros shall be appended at the end of the FECFRAME after FEC encoding.

5.3.3

Bit interleaver

See ETSI EN 302 307-1 [3], clause 5.3.3.

Bit interleaving is applied to all MODCODs except those using BPSK or QPSK. Table 9a describes the bit interleaver
setting for normal and medium FECFRAMES, Table 9b for short FECFRAMES. The write-in operation of the bit
interleaver follows the description of ETSI EN 302 307-1 [3], clause 5.3.3, i.e. data is serially written into the
interleaver column-wise. The rows are read out serially, but in an order described by the Bit Interleaver Pattern. As an
example, the bit interleaver pattern 102 means that for each row, the middle entry (1) is read out first, followed by the
leftmost entry (0) and finally the rightmost entry (2).

Table 9a: Bit Interleaver Patterns (read out order - 0 corresponds to MSB,
i.e. leftmost column), Normal FECFRAME

Implementation MODCOD Name  Bit Interleaver Pattern

8PSK 23/36
8PSK 25/36
8PSK 13/18
4+12APSK 26/45
4+12APSK 3/5
8+8APSK 18/30
4+12APSK 28/45
4+12APSK 23/36
8+8APSK 20/30
4+12APSK 25/36
4+12APSK 13/18
4+12+16rbAPSK 2/3
8+16+20+20APSK 7/9
8+16+20+20APSK 4/5
8+16+20+20APSK 5/6
2+4+2APSK 100/180
2+4+2APSK 104/180
8+8APSK 90/180
8+8APSK 96/180
8+8APSK 100/180

ETSI

012
102
102
3201
3210
0123
3012
3021
0123
2310
3021
21430
201543
124053
421053
012
012
3210
2310
2301

26

ETSI EN 302 307-2 V1.4.1 (2024-08)

Implementation MODCOD Name  Bit Interleaver Pattern

4+12APSK 140/180
4+12APSK 154/180
4+8+4+16APSK 128/180
4+8+4+16APSK 132/180
4+8+4+16APSK 140/180
16+16+16+16APSK 128/180
4+12+20+28APSK 132/180
128APSK 135/180
128APSK 140/180
256APSK 116/180
256APSK 20/30
256APSK 124/180
256APSK 128/180
256APSK 22/30
256APSK 135/180

3210
0321
40312
40312
40213
305214
520143
4250316
4130256
40372156
01234567
46320571
75642301
01234567
50743612

Table 9b: Bit Interleaver Patterns (read out order - 0 corresponds to MSB,
i.e. leftmost column), Short FECFRAME

Implementation MODCOD Name
8PSK, 7/15
8PSK, 8/15
8PSK, 26/45
8PSK, 32/45
4+12APSK, 7/15
4+12APSK, 8/15
4+12APSK, 26/45
4+12APSK, 3/5
4+12APSK, 32/45
4+12+16rbAPSK APSK, 2/3
4+12+16rbAPSK APSK, 32/45

Bit Interleaver Pattern
102
102
102
012
2103
2103
2130
3201
0123
41230
10423

For 128APSK padding is introduced to have an integer number of constellation points and slots in a FECFRAME.
84 ones shall be appended at the bit interleaver output.

5.4

Constellations and Bit mapping

5.4.0  General aspects

See ETSI EN 302 307-1 [3], clause 5.4.

Each FECFRAME (which is a sequence of 64 800 bits for normal FECFRAME, or 16 200 bits for short FECFRAME,
or 32 400 bits for medium FECFRAME), shall be serial-to-parallel converted (parallelism level = η
π/2BPSK; 2 for QPSK, 3 for 8PSK, 4 for 16APSK, 5 for 32APSK, 6 for 64APSK, 7 for 128APSK, 8 for 256APSK). In
Figures 5 to 15, the MSB of the FECFRAME is mapped into the MSB of the first parallel sequence. Each parallel
sequence shall be mapped into constellation, generating an (I,Q) sequence of variable length depending on the selected
modulation efficiency η

MOD 1 for

MOD.

For 128APSK padding is introduced to have an integer number of constellation points in a FECFRAME as stated in
clause 5.3.2.2. Thus, 6 zeros shall be appended at the end of the FECFRAME after FEC encoding.

NOTE:  The optimum constellation ring ratios given in the following are optimized for the AWGN channel. For

non-linear channels, ring ratios may be jointly optimized with the characteristics of non-linear pre-
distortion devices in the uplink station, for the selected operating point (IBO-OBO) of the non-linear
channel amplifier(s). Decoders may assume that the centroids of the received constellations, after suitable
AGC correction, are placed in the nominal positions as reported in the present document.

ETSI

27

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.4.0a  Bit mapping into π/2BPSK constellation (VL-SNR modes and

VL-SNR Header)

VL-SNR modes shall include π/2BPSK modulation. For "Spreading Factor 2" modes, FECFRAME bits shall be
repeated twice before mapping into constellation.

π/2BPSK symbols shall be generated according to the rule:

I2i-1 = Q2i-1 = (1/ 2 ) (1-2y2i-1), I2i = - Q2i = - (1/ 2 ) (1-2y2i) for i = 1, 2, ..., N

where N= nldpc/2 for π/2BPSK modes, N= nldpc for π/2BPSK Spreading Factor 2 modes, and N=450 for VL-SNR
header.

5.4.1

Bit mapping into QPSK constellation

See ETSI EN 302 307-1 [3], clause 5.4.1.

5.4.2

Bit mapping into 8PSK and 8APSK constellations

See ETSI EN 302 307-1 [3], clause 5.4.2.

Constellations with 8 points can be 8PSK (equal to 8PSK constellation in ETSI EN 302 307-1 [3]) and 8APSK, with
constellation points on 3 rings, 2 on the 1st ring, 4 on the 2nd ring and 2 on the 3rd ring (2+4+2). Tables 10a and 10b
indicate for 2+4+2APSK the constellation and label definition and the optimum constellation radius ratios for the code
identifiers it applies, respectively.

Table 10a: Constellation and label definition for 2+4+2APSK

Label

Radius

p00
p01
p10
p11

R1
R2
R2
R3

φ/π
p=0

1
1 + 0,352
1 - 0,352
1

φ/π
p=1

0
-0,352
0,352
0

Table 10b: Optimum Constellation Radius Ratios for 2+4+2APSK [γ

1 = R2/ R1, γ

2 = R3/ R1 ]

LDPC code identifier  Modulation/coding spectral efficiency

100/180
104/180

1,66
1,73

γ

1
5,32
6,39

γ2
6,8
8,0

ETSI

28

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 5: 2+4+2APSK Constellation (code rate 100/180)

5.4.3

Bit mapping into 16APSK constellation

See ETSI EN 302 307-1 [3], clause 5.4.3.

In addition to the 16APSK constellation defined in ETSI EN 302 307-1 [3], clause 5.4.3, that has 4 points on the first
ring and 12 on the second ring (4+12), another constellation is defined, with 8 points on the first ring and 8 points on
the second ring (8+8), Tables 11a and 11b indicate the optimum constellation radius ratios for 4+12APSK (the
constellation and label definition is identical to the 16APSK constellation defined in ETSI EN 302 307-1 [3]);
Tables 11c to 11e indicate for the 8+8APSK constellation the optimum constellation radius ratios for the code identifier
they apply, and the constellation and label definition.

Table 11a: Optimum Constellation Radius Ratio

γ

 for 4+12APSK

Normal FECFRAME

LDPC code identifier
26/45
3/5
28/45
23/36
25/36
13/18
140/180
154/180

Modulation/Coding Spectral Efficiency
2,30
2,39
2,48
2,54
2,77
2,88
3,11
3,42

γ

3,7
3,7
3,5
3,1
3,1
2,85
3,6
3,2

Table 11b: Optimum Constellation Radius Ratio

γ

 for 4+12APSK

Short FECFRAME

LDPC code identifier
7/15
8/15
26/45
3/5
32/45

Modulation/Coding Spectral Efficiency
1,83
2,09
2,27
2,36
2,80

Ring Ratios
3,32
3,50
3,7
3,7
2,85

ETSI

29

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 11c: Constellation and label definition for 8+8APSK
Normal FECFRAME, LDPC code identifiers 90/180, 96/180 and 100/180

Label

Radius

0qp0
0qp1
1qp0
1qp1

R1
R1
R2
R2

φ/π
p=q=0
1/8
3/8
1/8
3/8

φ/π
p=0, q=1
15/8
13/8
15/8
13/8

φ/π
p=1, q=0
7/8
5/8
7/8
5/8

φ/π
p=q=1
9/8
11/8
9/8
11/8

Table 11d: Optimum Constellation Radius Ratio for 8+8APSK
Normal FECFRAME, LDPC code identifiers 90/180, 96/180 and 100/180

LDPC code
identifier
90/180
96/180
100/180

Modulation/coding spectral efficiency

2,00
2,13
2,22

γ

2,19
2,19
2,19

Table 11e: Constellation and label definition for 8+8APSK
Normal FECFRAME, LDPC code identifiers 18/30 and 20/30

Label

0000
0001
0010
0011
0100
0101
0110
0111
1000
1001
1010
1011
1100
1101
1110
1111

Complex constellation point for LDPC
code identifier 18/30
0,4718 + 0,2606i
0,2606 + 0,4718i
-0,4718 + 0,2606i
-0,2606 + 0,4718i
0,4718 - 0,2606i
0,2606 - 0,4718i
-0,4718 - 0,2606i
-0,2606 - 0,4718i
1,2088 + 0,4984i
0,4984 + 1,2088i
-1,2088 + 0,4984i
-0,4984 + 1,2088i
1,2088 - 0,4984i
0,4984 - 1,2088i
-1,2088 - 0,4984i
-0,4984 - 1,2088i

Complex constellation point for LDPC
code identifier 20/30
0,5061 + 0,2474i
0,2474 + 0,5061i
-0,5061 + 0,2474i
-0,2474 + 0,5061i
0,5061 - 0,2474i
0,2474 - 0,5061i
-0,5061 - 0,2474i
-0,2474 - 0,5061i
1,2007 + 0,4909i
0,4909 + 1,2007i
-1,2007 + 0,4909i
-0,4909 + 1,2007i
1,2007 - 0,4909i
0,4909 - 1,2007i
-1,2007 - 0,4909i
-0,4909 - 1,2007i

ETSI

30

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 6: 8+8APSK Constellation (code rate 90/180)

Figure 7: 8+8APSK Constellation (code rate 18/30)

5.4.4

Bit mapping into 32APSK constellations

See ETSI EN 302 307-1 [3], clause 5.4.4.

In addition to the 32APSK constellation defined in ETSI EN 302 307-1 [3], clause 5.4.4, that has 4 points on the first
ring, 12 on the second ring and 16 on the third ring (4+12+16), a further constellation is introduced with 4 points on the
first ring, 12 on the second ring and 16 on the third ring (4+12+16), and another constellation, with 4 rings and 4 points
on the first ring, 8 on the second ring, 4 on the third ring and 16 on the fourth ring (4+8+4+16), Tables 12a to 12e
indicate for the two additional constellations with 32 points the optimum constellation radius ratios for the code
identifier they apply, and the constellation and label definition.

ETSI

31

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 12a: Optimum Constellation Radius Ratio
Normal FECFRAME

γ

γ

1 and

2 for 4+12+16rbAPSK

LDPC code identifier

Spectral Efficiency

2/3

3,32

γ

1
2,85

γ

2
5,55

Table 12b: Optimum Constellation Radius Ratio
Short FECFRAME

γ

γ

1 and

2 for 4+12+16rbAPSK

LDPC code identifier

Spectral Efficiency

2/3
32/45

3,28
3,50

γ

1
2,84
2,84

γ

2
5,54
5,26

Table 12c: Constellation and label definition for 4+12+16rbAPSK

Label

p00q0
p00q1
p01q0
p01q1
p10q0
p10q1
p11q0
p11q1

Radius

R3
R3
R2
R2
R3
R3
R2
R1

φ/π
p=q=0
11/16
9/16
3/4
7/12
13/16
15/16
11/12
3/4

φ/π
p=0, q=1
5/16
7/16
1/4
5/12
3/16
1/16
1/12
1/4

φ/π
p=1, q=0
21/16
23/16
5/4
17/12
19/16
17/16
13/12
5/4

φ/π
p=q=1
27/16
25/16
7/4
19/12
29/16
31/16
23/12
7/4

Figure 8: 4+12+16rbAPSK Constellation (code rate 2/3, Normal FECFRAME)

ETSI

32

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 12d: Constellation and label definition for 4+8+4+16APSK

Label

00pq0
00pq1
01pq0
01pq1
10pq0
10pq1
11pq0
11pq1

Radius

R1
R4
R2
R4
R2
R4
R3
R4

φ/π
p=q=0
1/4
7/16
1/12
1/16
5/12
5/16
1/4
3/16

φ/π
p=0, q=1
7/4
25/16
23/12
31/16
19/12
27/16
7/4
29/16

φ/π
p=1, q=0
3/4
9/16
11/12
15/16
7/12
11/16
3/4
13/16

φ/π
p=q=1
5/4
23/16
13/12
17/16
17/12
21/16
5/4
19/16

Table 12e: Optimum Constellation Radius Ratios for 4+8+4+16APSK
γ
2 = R3/R1 and

γ
1 = R2/R1,

3 = R4/R1]

γ

[

LDPC code identifier  Modulation/coding spectral efficiency

128/180
132/180
140/180

3,56
3,67
3,89

γ

1
2,6
2,6
2,8

γ2
2,99
2,86
3,08

γ3
5,6
5,6
5,6

Figure 9: 4+8+4+16APSK Constellation (code rate 128/180)

5.4.5

Bit mapping into 64APSK constellations

Three different 64APSK constellations are introduced, the first with 16 points on the first ring, 16 on the second ring,
16 on the third ring and 16 on the fourth ring (16+16+16+16), the second with 8 points on the first ring, 16 on the
second ring, 20 on the third ring and 20 on the fourth ring (8+16+20+20), the third with 4 points on the first ring, 12 on
the second ring, 20 on the third ring and 28 on the fourth ring (4+12+20+28). Tables 13a to 13f indicate for the three
constellations with 64 points the optimum constellation radius ratios for the code identifier they apply, and the
constellation and label definition.

ETSI

33

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 13a: Constellation and label definition for 16+16+16+16APSK

Label

Radius

00qp00
00qp01
00qp10
00qp11
01qp00
01qp01
01qp10
01qp11
10qp00
10qp01
10qp10
10qp11
11qp00
11qp01
11qp10
11qp11

R1
R1
R1
R1
R2
R2
R2
R2
R4
R4
R4
R4
R3
R3
R3
R3

φ/π
p=q=0
1/16
3/16
7/16
5/16
1/16
3/16
7/16
5/16
1/16
3/16
7/16
5/16
1/16
3/16
7/16
5/16

φ/π
p=0, q=1
31/16
29/16
25/16
27/16
31/16
29/16
25/16
27/16
31/16
29/16
25/16
27/16
31/16
29/16
25/16
27/16

φ/π
p=1, q=0
15/16
13/16
9/16
11/16
15/16
13/16
9/16
11/16
15/16
13/16
9/16
11/16
15/16
13/16
9/16
11/16

φ/π
p=q=1
17/16
19/16
23/16
21/16
17/16
19/16
23/16
21/16
17/16
19/16
23/16
21/16
17/16
19/16
23/16
21/16

Table 13b: Optimum Constellation Radius Ratios for 16+16+16+16APSK
γ
2 = R3/R1 and

γ
1 = R2/R1,

3 = R4/R1]

γ
[

LDPC code identifier  Modulation/coding spectral efficiency

128/180

4,27

γ

1
1,88

γ2
2,72

γ3
3,95

Figure 10: 16+16+16+16APSK Constellation (code rate 128/180)

ETSI

34

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 13c: Constellation and label definition for 8+16+20+20APSK

Label

Radius

p0q000
p0q001
p0q010
p0q011
p0q100
p0q101
p0q110
p0q111
p1q000
p1q001
p1q010
p1q011
p1q100
p1q101
p1q110
p1q111

R2
R4
R2
R3
R4
R4
R3
R3
R1
R4
R2
R3
R1
R4
R2
R3

φ/π
p=q=0
25/16
7/4
27/16
7/4
31/20
33/20
31/20
33/20
13/8
37/20
29/16
37/20
15/8
39/20
31/16
39/20

φ/π
p=0, q=1
23/16
5/4
21/16
5/4
29/20
27/20
29/20
27/20
11/8
23/20
19/16
23/20
9/8
21/20
17/16
21/20

φ/π
p=1, q=0
7/16
1/4
5/16
1/4
9/20
7/20
9/20
7/20
3/8
3/20
3/16
3/20
1/8
1/20
1/16
1/20

φ/π
p=q=1
9/16
3/4
11/16
3/4
11/20
13/20
11/20
13/20
5/8
17/20
13/16
17/20
7/8
19/20
15/16
19/20

Table 13d: Optimum Constellation Radius Ratios for 8+16+20+20APSK
γ
2 = R3/R1 and

γ
1 = R2/R1,

3 = R4/R1]

γ

[

LDPC code identifier  Modulation/coding spectral efficiency

7/9
4/5
5/6

4,65
4,78
4,98

γ

1
2,2
2,2
2,2

γ2
3,6
3,6
3,5

γ3
5,2
5,2
5,0

Figure 11: 8+16+20+20APSK Constellation (code rate 7/9)

ETSI

35

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 13e: Constellation and label definition for 4+12+20+28APSK

Label

Radius

0000pq
0001pq
0010pq
0011pq
0100pq
0101pq
0110pq
0111pq
1000pq
1001pq
1010pq
1011pq
1100pq
1101pq
1110pq
1111pq

R4
R4
R4
R1
R4
R4
R3
R2
R4
R3
R4
R2
R3
R3
R3
R2

φ/π
p=q=0
1/4
13/28
1/28
1/4
9/28
11/28
1/20
1/12
5/28
9/20
3/28
5/12
1/4
7/20
3/20
1/4

φ/π
p=0, q=1
7/4
43/28
55/28
7/4
47/28
45/28
39/20
23/12
51/28
31/20
53/28
19/12
7/4
33/20
37/20
7/4

φ/π
p=1, q=0
3/4
15/28
27/28
3/4
19/28
17/28
19/20
11/12
23/28
11/20
25/28
7/12
3/4
13/20
17/20
3/4

φ/π
p=q=1
5/4
41/28
29/28
5/4
37/28
39/28
21/20
13/12
33/28
29/20
31/28
17/12
5/4
27/20
23/20
5/4

Table 13f: Optimum Constellation Radius Ratios for 4+12+20+28APSK
γ
2 = R3/R1 and

γ
1 = R2/R1,

3 = R4/R1]

γ

[

LDPC code identifier  Modulation/coding spectral efficiency

132/180

4,40

γ

1
2,4

γ2
4,3

γ3
7

Figure 12: 4+12+20+28APSK Constellation (code rate 132/180)

5.4.6

Bit mapping into 128APSK constellations

One 128APSK constellation is introduced, with 6 rings and 128 constellation points. Tables 14a and 14b indicate the
optimum constellation radius ratios for the code identifier they apply, and the constellation and label definition.

ETSI

36

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 14a: Optimum Constellation Radius Ratios for 128APSK
γ
3 = R4/R1,

γ
4 = R5/R1,

γ
1 = R2/R1,

γ
2 = R3/R1,

5 = R6/R1]

γ

[

LDPC code
identifier
135/180
140/180

Modulation/coding
spectral efficiency
5,25
5,44

γ

1

1,715
1,715

γ2

2,118
2,118

γ3

2,681
2,681

γ4

2,75
2,75

γ5

3,819
3,733

Table 14b: Constellation and label definition for 128APSK

Label

Radius

qp00000
qp00001
qp00010
qp00011
qp00100
qp00101
qp00110
qp00111
qp01000
qp01001
qp01010
qp01011
qp01100
qp01101
qp01110
qp01111
qp10000
qp10001
qp10010
qp10011
qp10100
qp10101
qp10110
qp10111
qp11000
qp11001
qp11010
qp11011
qp11100
qp11101
qp11110
qp11111

R1
R6
R6
R6
R2
R3
R5
R4
R1
R6
R6
R6
R2
R3
R5
R4
R1
R6
R6
R6
R2
R3
R5
R4
R1
R6
R6
R6
R2
R3
R5
R4

φ/π
p=q=0
83/1260
11/105
37/1680
11/168
121/2520
23/280
19/720
61/720
103/560
61/420
383/1680
929/5040
113/560
169/1008
563/2520
139/840
243/560
1993/5040
43/90
73/168
1139/2520
117/280
341/720
349/840
177/560
1789/5040
49/180
53/168
167/560
239/720
199/720
281/840

φ/π
p=0, q=1
2437/1260
199/105
3323/1680
325/168
4919/2520
537/280
1421/720
1379/720
1017/560
779/420
2977/1680
9151/5040
1007/560
1847/1008
4477/2520
1541/840
877/560
8087/5040
137/90
263/168
3901/2520
443/280
1099/720
1331/840
943/560
8291/5040
311/180
283/168
953/560
1201/720
1241/720
1399/840

φ/π
p=1, q=0
1177/1260
94/105
1643/1680
157/168
2399/2520
257/280
701/720
659/720
457/560
359/420
1297/1680
4111/5040
447/560
839/1008
1957/2520
701/840
317/560
3047/5040
47/90
95/168
1381/2520
163/280
379/720
491/840
383/560
3251/5040
131/180
115/168
393/560
481/720
521/720
559/840

φ/π
p=q=1
1343/1260
116/105
1717/1680
179/168
2641/2520
303/280
739/720
781/720
663/560
481/420
2063/1680
5969/5040
673/560
1177/1008
3083/2520
979/840
803/560
7033/5040
133/90
241/168
3659/2520
397/280
1061/720
1189/840
737/560
6829/5040
229/180
221/168
727/560
959/720
919/720
1121/840

ETSI

37

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 13: 128APSK Constellation (code rate 135/180)

5.4.7

Bit mapping into 256APSK constellations

Two different 256APSK constellations are introduced, with 256 constellation points. Tables 15a to 15d indicate for the
two constellations with 256 points the optimum constellation radius ratios for the code identifier they apply, or the
coordinates of the constellation points, and the constellation and label definition.

γ

[

γ
1 = R2/R1,

Table 15a: Optimum Constellation Radius Ratios for 256APSK
γ
4 = R5/R1,

γ
5 = R6/R1,

γ
3 = R4/R1,

γ
6 = R7/R1,

γ
2 = R3/R1,

7 = R8/R1]

LDPC code
identifier
116/180
124/180
128/180
135/180

Modulation/coding
spectral efficiency
5,16
5,51
5,69
6,00

γ

1

1,791
1,791
1,794
1,794

γ2

γ3

γ4

γ5

γ6

γ7

2,405
2,405
2,409
2,409

2,980
2,980
2,986
2,986

3,569
3,569
3,579
3,579

4,235
4,235
4,045
4,045

5,078
5,078
4,6
4,5

6,536
6,536
5,4
5,2

Table 15b: Constellation and label definition for 256APSK (Ring radii)

Label
000qpaaa
001qpaaa
010qpaaa
011qpaaa
100qpaaa
101qpaaa
110qpaaa
111qpaaa

Radius
R1
R2
R4
R3
R8
R7
R5
R6

ETSI

Table 15c: Constellation and label definition for 256APSK (Constellation points angles)

38

ETSI EN 302 307-2 V1.4.1 (2024-08)

Label

rrrqp000

rrrqp001

rrrqp010

rrrqp011

rrrqp100

rrrqp101

rrrqp110

rrrqp111

φ/π
p=q=0
φ
1 = 1π/32
2 = 3π/32
φ
φ
4 = 7π/32
3 = 5π/32
φ
8 = 15π/32
φ
7 = 13π/32
φ
5 = 9π/32
φ
6 = 11π/32
φ

φ/π
p=0, q=1
-φ
1
-φ
2
-φ
4
-φ
3
-φ
8
-φ
7
-φ
5
-φ
6

φ/π
p=1, q=0
π-φ
π-φ
π-φ
π-φ
π-φ
π-φ
π-φ
π-φ

1
2
4
3
8
7
5
6

φ/π
p=q=1
π+φ
1
π+φ
2
π+φ
4
π+φ
3
π+φ
8
π+φ
7
π+φ
5
π+φ
6

Figure 14: 256APSK Constellation (code rate 116/180)

Table 15d: Optimum Constellation for 256APSK for LDPC code identifiers 20/30 and 22/30

Label

00000000
00000001
00000010
00000011
00000100
00000101
00000110
00000111
00001000
00001001
00001010
00001011
00001100
00001101
00001110
00001111

Complex constellation point for LDPC
code identifier 20/30
1,6350 + 0,1593i
1,5776 + 0,4735i
0,9430 + 0,1100i
0,9069 + 0,2829i
0,3237 + 0,0849i
0,3228 + 0,0867i
0,7502 + 0,1138i
0,7325 + 0,2088i
0,1658 + 1,6747i
0,4907 + 1,6084i
0,1088 + 0,9530i
0,2464 + 0,9270i
0,0872 + 0,1390i
0,0871 + 0,1392i
0,1091 + 0,7656i
0,1699 + 0,7537i

Complex constellation point for LDPC
code identifier 22/30
1,5977 + 0,1526i
1,3187 + 0,1269i
-1,5977 + 0,1526i
-1,3187 + 0,1269i
0,2574 + 0,0733i
0,4496 + 0,0807i
-0,2574 + 0,0733i
-0,4496 + 0,0807i
1,5977 - 0,1526i
1,3187 - 0,1269i
-1,5977 - 0,1526i
-1,3187 - 0,1269i
0,2574 - 0,0733i
0,4496 - 0,0807i
-0,2574 - 0,0733i
-0,4496 - 0,0807i

ETSI

39

ETSI EN 302 307-2 V1.4.1 (2024-08)

Label

00010000
00010001
00010010
00010011
00010100
00010101
00010110
00010111
00011000
00011001
00011010
00011011
00011100
00011101
00011110
00011111
00100000
00100001
00100010
00100011
00100100
00100101
00100110
00100111
00101000
00101001
00101010
00101011
00101100
00101101
00101110
00101111
00110000
00110001
00110010
00110011
00110100
00110101
00110110
00110111
00111000
00111001
00111010
00111011
00111100
00111101
00111110
00111111
01000000
01000001
01000010
01000011
01000100
01000101
01000110
01000111
01001000
01001001
01001010
01001011
01001100
01001101
01001110

Complex constellation point for LDPC
code identifier 20/30
-1,6350 + 0,1593i
-1,5776 + 0,4735i
-0,9430 + 0,1100i
-0,9069 + 0,2829i
-0,3237 + 0,0849i
-0,3228 + 0,0867i
-0,7502 + 0,1138i
-0,7325 + 0,2088i
-0,1658 + 1,6747i
-0,4907 + 1,6084i
-0,1088 + 0,9530i
-0,2464 + 0,9270i
-0,0872 + 0,1390i
-0,0871 + 0,1392i
-0,1091 + 0,7656i
-0,1699 + 0,7537i
1,3225 + 0,1320i
1,2742 + 0,3922i
1,0854 + 0,1139i
1,0441 + 0,3296i
0,4582 + 0,1123i
0,4545 + 0,1251i
0,6473 + 0,1138i
0,6339 + 0,1702i
0,1322 + 1,3631i
0,3929 + 1,3102i
0,1124 + 1,1327i
0,3160 + 1,0913i
0,0928 + 0,3970i
0,0937 + 0,3973i
0,1054 + 0,5979i
0,1230 + 0,5949i
-1,3225 + 0,1320i
-1,2742 + 0,3922i
-1,0854 + 0,1139i
-1,0441 + 0,3296i
-0,4582 + 0,1123i
-0,4545 + 0,1251i
-0,6473 + 0,1138i
-0,6339 + 0,1702i
-0,1322 + 1,3631i
-0,3929 + 1,3102i
-0,1124 + 1,1327i
-0,3160 + 1,0913i
-0,0928 + 0,3970i
-0,0937 + 0,3973i
-0,1054 + 0,5979i
-0,1230 + 0,5949i
1,6350 - 0,1593i
1,5776 - 0,4735i
0,9430 - 0,1100i
0,9069 - 0,2829i
0,3237 - 0,0849i
0,3228 - 0,0867i
0,7502 - 0,1138i
0,7325 - 0,2088i
0,1658 - 1,6747i
0,4907 - 1,6084i
0,1088 - 0,9530i
0,2464 - 0,9270i
0,0872 - 0,1390i
0,0871 - 0,1392i
0,1091 - 0,7656i

Complex constellation point for LDPC
code identifier 22/30
0,9269 + 0,0943i
1,1024 + 0,1086i
-0,9269 + 0,0943i
-1,1024 + 0,1086i
0,7663 + 0,0867i
0,6115 + 0,0871i
-0,7663 + 0,0867i
-0,6115 + 0,0871i
0,9269 - 0,0943i
1,1024 - 0,1086i
-0,9269 - 0,0943i
-1,1024 - 0,1086i
0,7663 - 0,0867i
0,6115 - 0,0871i
-0,7663 - 0,0867i
-0,6115 - 0,0871i
1,2701 + 1,0139i
1,0525 + 0,8406i
-1,2701 + 1,0139i
-1,0525 + 0,8406i
0,2487 + 0,1978i
0,3523 + 0,2915i
-0,2487 + 0,1978i
-0,3523 + 0,2915i
1,2701 - 1,0139i
1,0525 - 0,8406i
-1,2701 - 1,0139i
-1,0525 - 0,8406i
0,2487 - 0,1978i
0,3523 - 0,2915i
-0,2487 - 0,1978i
-0,3523 - 0,2915i
0,7359 + 0,6043i
0,8807 + 0,7105i
-0,7359 + 0,6043i
-0,8807 + 0,7105i
0,6017 + 0,5019i
0,4747 + 0,3996i
-0,6017 + 0,5019i
-0,4747 + 0,3996i
0,7359 - 0,6043i
0,8807 - 0,7105i
-0,7359 - 0,6043i
-0,8807 - 0,7105i
0,6017 - 0,5019i
0,4747 - 0,3996i
-0,6017 - 0,5019i
-0,4747 - 0,3996i
1,5441 + 0,4545i
1,2750 + 0,3775i
-1,5441 + 0,4545i
-1,2750 + 0,3775i
0,2586 + 0,0752i
0,4435 + 0,1065i
-0,2586 + 0,0752i
-0,4435 + 0,1065i
1,5441 - 0,4545i
1,2750 - 0,3775i
-1,5441 - 0,4545i
-1,2750 - 0,3775i
0,2586 - 0,0752i
0,4435 - 0,1065i
-0,2586 - 0,0752i

ETSI

40

ETSI EN 302 307-2 V1.4.1 (2024-08)

Label

01001111
01010000
01010001
01010010
01010011
01010100
01010101
01010110
01010111
01011000
01011001
01011010
01011011
01011100
01011101
01011110
01011111
01100000
01100001
01100010
01100011
01100100
01100101
01100110
01100111
01101000
01101001
01101010
01101011
01101100
01101101
01101110
01101111
01110000
01110001
01110010
01110011
01110100
01110101
01110110
01110111
01111000
01111001
01111010
01111011
01111100
01111101
01111110
01111111
10000000
10000001
10000010
10000011
10000100
10000101
10000110
10000111
10001000
10001001
10001010
10001011
10001100
10001101

Complex constellation point for LDPC
code identifier 20/30
0,1699 - 0,7537i
-1,6350 - 0,1593i
-1,5776 - 0,4735i
-0,9430 - 0,1100i
-0,9069 - 0,2829i
-0,3237 - 0,0849i
-0,3228 - 0,0867i
-0,7502 - 0,1138i
-0,7325 - 0,2088i
-0,1658 - 1,6747i
-0,4907 - 1,6084i
-0,1088 - 0,9530i
-0,2464 - 0,9270i
-0,0872 - 0,1390i
-0,0871 - 0,1392i
-0,1091 - 0,7656i
-0,1699 - 0,7537i
1,3225 - 0,1320i
1,2742 - 0,3922i
1,0854 - 0,1139i
1,0441 - 0,3296i
0,4582 - 0,1123i
0,4545 - 0,1251i
0,6473 - 0,1138i
0,6339 - 0,1702i
0,1322 - 1,3631i
0,3929 - 1,3102i
0,1124 - 1,1327i
0,3160 - 1,0913i
0,0928 - 0,3970i
0,0937 - 0,3973i
0,1054 - 0,5979i
0,1230 - 0,5949i
-1,3225 - 0,1320i
-1,2742 - 0,3922i
-1,0854 - 0,1139i
-1,0441 - 0,3296i
-0,4582 - 0,1123i
-0,4545 - 0,1251i
-0,6473 - 0,1138i
-0,6339 - 0,1702i
-0,1322 - 1,3631i
-0,3929 - 1,3102i
-0,1124 - 1,1327i
-0,3160 - 1,0913i
-0,0928 - 0,3970i
-0,0937 - 0,3973i
-0,1054 - 0,5979i
-0,1230 - 0,5949i
1,2901 + 1,0495i
1,4625 + 0,7740i
0,7273 + 0,6160i
0,8177 + 0,4841i
0,2844 + 0,1296i
0,2853 + 0,1309i
0,5902 + 0,4857i
0,6355 + 0,4185i
1,0646 + 1,2876i
0,7949 + 1,4772i
0,5707 + 0,7662i
0,4490 + 0,8461i
0,1053 + 0,1494i
0,1052 + 0,1495i

Complex constellation point for LDPC
code identifier 22/30
-0,4435 - 0,1065i
0,8925 + 0,2771i
1,0649 + 0,3219i
-0,8925 + 0,2771i
-1,0649 + 0,3219i
0,7362 + 0,2279i
0,5936 + 0,1699i
-0,7362 + 0,2279i
-0,5936 + 0,1699i
0,8925 - 0,2771i
1,0649 - 0,3219i
-0,8925 - 0,2771i
-1,0649 - 0,3219i
0,7362 - 0,2279i
0,5936 - 0,1699i
-0,7362 - 0,2279i
-0,5936 - 0,1699i
1,4352 + 0,7452i
1,1866 + 0,6182i
-1,4352 + 0,7452i
-1,1866 + 0,6182i
0,2523 + 0,1944i
0,3695 + 0,2695i
-0,2523 + 0,1944i
-0,3695 + 0,2695i
1,4352 - 0,7452i
1,1866 - 0,6182i
-1,4352 - 0,7452i
-1,1866 - 0,6182i
0,2523 - 0,1944i
0,3695 - 0,2695i
-0,2523 - 0,1944i
-0,3695 - 0,2695i
0,8273 + 0,4493i
0,9911 + 0,5243i
-0,8273 + 0,4493i
-0,9911 + 0,5243i
0,6708 + 0,3859i
0,5197 + 0,3331i
-0,6708 + 0,3859i
-0,5197 + 0,3331i
0,8273 - 0,4493i
0,9911 - 0,5243i
-0,8273 - 0,4493i
-0,9911 - 0,5243i
0,6708 - 0,3859i
0,5197 - 0,3331i
-0,6708 - 0,3859i
-0,5197 - 0,3331i
0,1646 + 1,6329i
0,1379 + 1,3595i
-0,1646 + 1,6329i
-0,1379 + 1,3595i
0,0736 + 0,0898i
0,0742 + 0,5054i
-0,0736 + 0,0898i
-0,0742 + 0,5054i
0,1646 - 1,6329i
0,1379 - 1,3595i
-0,1646 - 1,6329i
-0,1379 - 1,3595i
0,0736 - 0,0898i
0,0742 - 0,5054i

ETSI

41

ETSI EN 302 307-2 V1.4.1 (2024-08)

Label

10001110
10001111
10010000
10010001
10010010
10010011
10010100
10010101
10010110
10010111
10011000
10011001
10011010
10011011
10011100
10011101
10011110
10011111
10100000
10100001
10100010
10100011
10100100
10100101
10100110
10100111
10101000
10101001
10101010
10101011
10101100
10101101
10101110
10101111
10110000
10110001
10110010
10110011
10110100
10110101
10110110
10110111
10111000
10111001
10111010
10111011
10111100
10111101
10111110
10111111
11000000
11000001
11000010
11000011
11000100
11000101
11000110
11000111
11001000
11001001
11001010
11001011
11001100

Complex constellation point for LDPC
code identifier 20/30
0,4294 + 0,6363i
0,3744 + 0,6744i
-1,2901 + 1,0495i
-1,4625 + 0,7740i
-0,7273 + 0,6160i
-0,8177 + 0,4841i
-0,2844 + 0,1296i
-0,2853 + 0,1309i
-0,5902 + 0,4857i
-0,6355 + 0,4185i
-1,0646 + 1,2876i
-0,7949 + 1,4772i
-0,5707 + 0,7662i
-0,4490 + 0,8461i
-0,1053 + 0,1494i
-0,1052 + 0,1495i
-0,4294 + 0,6363i
-0,3744 + 0,6744i
1,0382 + 0,8623i
1,1794 + 0,6376i
0,8504 + 0,7217i
0,9638 + 0,5407i
0,3734 + 0,2560i
0,3799 + 0,2517i
0,4968 + 0,3947i
0,5231 + 0,3644i
0,8555 + 1,0542i
0,6363 + 1,2064i
0,6961 + 0,8850i
0,5229 + 1,0037i
0,1938 + 0,3621i
0,1909 + 0,3627i
0,3224 + 0,5236i
0,3016 + 0,5347i
-1,0382 + 0,8623i
-1,1794 + 0,6376i
-0,8504 + 0,7217i
-0,9638 + 0,5407i
-0,3734 + 0,2560i
-0,3799 + 0,2517i
-0,4968 + 0,3947i
-0,5231 + 0,3644i
-0,8555 + 1,0542i
-0,6363 + 1,2064i
-0,6961 + 0,8850i
-0,5229 + 1,0037i
-0,1938 + 0,3621i
-0,1909 + 0,3627i
-0,3224 + 0,5236i
-0,3016 + 0,5347i
1,2901 - 1,0495i
1,4625 - 0,7740i
0,7273 - 0,6160i
0,8177 - 0,4841i
0,2844 - 0,1296i
0,2853 - 0,1309i
0,5902 - 0,4857i
0,6355 - 0,4185i
1,0646 - 1,2876i
0,7949 - 1,4772i
0,5707 - 0,7662i
0,4490 - 0,8461i
0,1053 - 0,1494i

Complex constellation point for LDPC
code identifier 22/30
-0,0736 - 0,0898i
-0,0742 - 0,5054i
0,0992 + 0,9847i
0,1170 + 1,1517i
-0,0992 + 0,9847i
-0,1170 + 1,1517i
0,0894 + 0,8287i
0,0889 + 0,6739i
-0,0894 + 0,8287i
-0,0889 + 0,6739i
0,0992 - 0,9847i
0,1170 - 1,1517i
-0,0992 - 0,9847i
-0,1170 - 1,1517i
0,0894 - 0,8287i
0,0889 - 0,6739i
-0,0894 - 0,8287i
-0,0889 - 0,6739i
1,0516 + 1,2481i
0,8742 + 1,0355i
-1,0516 + 1,2481i
-0,8742 + 1,0355i
0,0970 + 0,2450i
0,1959 + 0,4045i
-0,0970 + 0,2450i
-0,1959 + 0,4045i
1,0516 - 1,2481i
0,8742 - 1,0355i
-1,0516 - 1,2481i
-0,8742 - 1,0355i
0,0970 - 0,2450i
0,1959 - 0,4045i
-0,0970 - 0,2450i
-0,1959 - 0,4045i
0,6150 + 0,7441i
0,7345 + 0,8743i
-0,6150 + 0,7441i
-0,7345 + 0,8743i
0,4932 + 0,6301i
0,3620 + 0,5258i
-0,4932 + 0,6301i
-0,3620 + 0,5258i
0,6150 - 0,7441i
0,7345 - 0,8743i
-0,6150 - 0,7441i
-0,7345 - 0,8743i
0,4932 - 0,6301i
0,3620 - 0,5258i
-0,4932 - 0,6301i
-0,3620 - 0,5258i
0,4866 + 1,5660i
0,4068 + 1,3027i
-0,4866 + 1,5660i
-0,4068 + 1,3027i
0,0732 + 0,0899i
0,0877 + 0,4997i
-0,0732 + 0,0899i
-0,0877 + 0,4997i
0,4866 - 1,5660i
0,4068 - 1,3027i
-0,4866 - 1,5660i
-0,4068 - 1,3027i
0,0732 - 0,0899i

ETSI

42

ETSI EN 302 307-2 V1.4.1 (2024-08)

Label

11001101
11001110
11001111
11010000
11010001
11010010
11010011
11010100
11010101
11010110
11010111
11011000
11011001
11011010
11011011
11011100
11011101
11011110
11011111
11100000
11100001
11100010
11100011
11100100
11100101
11100110
11100111
11101000
11101001
11101010
11101011
11101100
11101101
11101110
11101111
11110000
11110001
11110010
11110011
11110100
11110101
11110110
11110111
11111000
11111001
11111010
11111011
11111100
11111101
11111110
11111111

Complex constellation point for LDPC
code identifier 20/30
0,1052 - 0,1495i
0,4294 - 0,6363i
0,3744 - 0,6744i
-1,2901 - 1,0495i
-1,4625 - 0,7740i
-0,7273 - 0,6160i
-0,8177 - 0,4841i
-0,2844 - 0,1296i
-0,2853 - 0,1309i
-0,5902 - 0,4857i
-0,6355 - 0,4185i
-1,0646 - 1,2876i
-0,7949 - 1,4772i
-0,5707 - 0,7662i
-0,4490 - 0,8461i
-0,1053 - 0,1494i
-0,1052 - 0,1495i
-0,4294 - 0,6363i
-0,3744 - 0,6744i
1,0382 - 0,8623i
1,1794 - 0,6376i
0,8504 - 0,7217i
0,9638 - 0,5407i
0,3734 - 0,2560i
0,3799 - 0,2517i
0,4968 - 0,3947i
0,5231 - 0,3644i
0,8555 - 1,0542i
0,6363 - 1,2064i
0,6961 - 0,8850i
0,5229 - 1,0037i
0,1938 - 0,3621i
0,1909 - 0,3627i
0,3224 - 0,5236i
0,3016 - 0,5347i
-1,0382 - 0,8623i
-1,1794 - 0,6376i
-0,8504 - 0,7217i
-0,9638 - 0,5407i
-0,3734 - 0,2560i
-0,3799 - 0,2517i
-0,4968 - 0,3947i
-0,5231 - 0,3644i
-0,8555 - 1,0542i
-0,6363 - 1,2064i
-0,6961 - 0,8850i
-0,5229 - 1,0037i
-0,1938 - 0,3621i
-0,1909 - 0,3627i
-0,3224 - 0,5236i
-0,3016 - 0,5347i

Complex constellation point for LDPC
code identifier 22/30
0,0877 - 0,4997i
-0,0732 - 0,0899i
-0,0877 - 0,4997i
0,2927 + 0,9409i
0,3446 + 1,1023i
-0,2927 + 0,9409i
-0,3446 + 1,1023i
0,2350 + 0,7945i
0,1670 + 0,6529i
-0,2350 + 0,7945i
-0,1670 + 0,6529i
0,2927 - 0,9409i
0,3446 - 1,1023i
-0,2927 - 0,9409i
-0,3446 - 1,1023i
0,2350 - 0,7945i
0,1670 - 0,6529i
-0,2350 - 0,7945i
-0,1670 - 0,6529i
0,7867 + 1,4356i
0,6561 + 1,1927i
-0,7867 + 1,4356i
-0,6561 + 1,1927i
0,0947 + 0,2451i
0,1865 + 0,4121i
-0,0947 + 0,2451i
-0,1865 + 0,4121i
0,7867 - 1,4356i
0,6561 - 1,1927i
-0,7867 - 1,4356i
-0,6561 - 1,1927i
0,0947 - 0,2451i
0,1865 - 0,4121i
-0,0947 - 0,2451i
-0,1865 - 0,4121i
0,4677 + 0,8579i
0,5537 + 1,0081i
-0,4677 + 0,8579i
-0,5537 + 1,0081i
0,3893 + 0,7143i
0,3110 + 0,5686i
-0,3893 + 0,7143i
-0,3110 + 0,5686i
0,4677 - 0,8579i
0,5537 - 1,0081i
-0,4677 - 0,8579i
-0,5537 - 1,0081i
0,3893 - 0,7143i
0,3110 - 0,5686i
-0,3893 - 0,7143i
-0,3110 - 0,5686i

ETSI

43

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 15: 256APSK Constellation (code rate 20/30)

5.5

Physical Layer (PL) framing

5.5.0  General aspects

See ETSI EN 302 307-1 [3], clause 5.5.

Table 16 (see Table 11 of ETSI EN 302 307-1 [3] for η

MOD = 2, 3, 4, 5):

S = number of SLOTs (M = 90 symbols) per XFECFRAME

η

MOD (bit/s/Hz)
0,5
1
2
3
4
5
6
7
8

S

nldpc = 64 800
(normal FECFRAME)
η % no-pilot
-
-
99,72
99,59
99,45
99,31
99,17
99,04
98,90

-
-
360
240
180
144
120
103
90

nldpc = 16 200
(short FECFRAME)
η % no-pilot
S
99,72
-
98,90
98,36
97,83
97,30
-
-
--

360
-
90
60
45
36
-
-
-

nldpc = 32 400
(medium FECFRAME)

S

-
360
-
-
-
-
-
-
-

η % no-pilot
-
99,72
-
-
-
-
-
-
-

5.5.1

Dummy PLFRAME insertion

See ETSI EN 302 307-1 [3], clause 5.5.1.

A Dummy PLFRAME shall be composed of a PLHEADER (see ETSI EN 302 307-1 [3], clause 5.5.2) and of 36
2)}.
2, +1/
SLOTS of 90 modulated symbols with (Ii,Qi)

2), (+1/

2), (-1/

2), (-1/

2, +1/

{(+1/

2, -1/

2, -1/

∈

√

√

√

√

√

√

√

√

ETSI

44

ETSI EN 302 307-2 V1.4.1 (2024-08)

NOTE:  The difference with ETSI EN 302 307-1 [3], clause 5.5.1 is that here the symbols are allowed to be

modulated by an arbitrary pseudo random sequence or any other sequence with similar spectral
properties. The PLS codes of the DUMMY PLFRAME remain identical to the PLS codes used in
ETSI EN 302 307-1 [3].

In the case of VL-SNR PLFRAMES, the VL-SNR Dummy PLFRAME shall be composed of:

1)

2)

3)

PLS header with code decimal value of 131;

followed by VL SNR HEADER (see clause 5.5.2.5);

followed by 15 696 unmodulated symbols (I,Q)=(+1/ 2 , +1/ 2 ).

5.5.2

PL signalling

5.5.2.0

General aspects

See ETSI EN 302 307-1 [3], clause 5.5.2.

In addition to conventional PLFRAME where a PLHEADER is appended to each XFECFRAME, S2X can transport
VL-SNR XFECFRAMEs (as defined in Table 18a). In this case, after the conventional PLHEADER, an additional
VL-SNR Header is transmitted.

/2-BPPSK Modulation

π

PLS Header 1 slot

VL SNR Header 10 slots

FEC Frame

SOF

PLS Code

PL Frame Before Pilot insertion and PL Scrambling

Figure 16: Insertion of VL-SNR Headers

VL-SNR-Header format is described in clause 5.5.2.5.

VL-SNR XFECFRAMEs shall be of two sets (see Table 18a):

•

•

Set 1 shall be characterized by XFECFRAMEs of 33 282 modulated symbols including the header and pilot
symbols.

Set 2 shall be characterized by XFECFRAMEs of 16 686 modulated symbols including the header and pilot
symbols.

In specific cases VL-SNR frames may be inserted in a S2 transmission without disturbing the regular reception of the
S2-frames by legacy receivers capable of ACM/VCM operation (these simply ignore the VL-SNR frames). In order to
make this feasible, the PLHEADERs of the VL-SNR frames shall indicate an un-used (by S2 services) MODCOD and
TYPE configuration, corresponding to the suitable XFECFRAME length (i.e. 32 400 symbols for VL-SNR-frames of
Set-1 or 16 200 symbols for Set-2).

For example, MODCOD QPSK 9/10 normal FECFRAME is suitable to transport VL-SNR frames of Set-1 while
MODCOD 16APSK 9/10 normal FECFRAME is suitable to transport VL-SNR frames of Set-2.

ETSI

45

ETSI EN 302 307-2 V1.4.1 (2024-08)

In addition to the regular 36 symbol pilots of S2-frames, VL-SNR frames shall insert additional pilot symbols which are
either 32, 34, or 36 symbols long as shown in Figures 17 and 18. In particular for VL-SNR frames of Set-1, additional
34 symbol pilots shall be inserted within the groups 1 through 18, and additional 36 symbol pilots shall be inserted
within the groups 19 through 21, as shown in Figure 17. For VL-SNR frames of Set-2, additional 32 symbol pilots shall
be inserted within the groups 1 through 9, and additional 36 symbol pilots shall be inserted within the group 10, as
shown in Figure 18.

Figure 17: VL-SNR XFECFRAME Set 1 with total length of 33 282 symbols,
the same as a QPSK normal length with pilot

Figure 18: VL-SNR XFECFRAME Set 2 with total length 16 686 symbols,
the same as 16APSK normal length with pilot

The PLHEADER (one SLOT of 90 symbols) shall be composed of the following fields:

•

•

SOF (26 symbols), identifying the Start of Frame.

PLS code (64 symbol): PLS (Physical Layer Signalling) code, carrying 1+7 signalling bits denoted as
(b0, b1, …, b7), where b0 is the Most Significant Bit (MSB) and b7 is the Least Significant Bit (LSB). The
most significant bit indicates whether the PL header refers to regular DVB-S2 MODCODs (b0 = 0) or whether
the PL header refers to MODCODs defined in the present document, (b0 = 1) under clause 5.5.2.2:

-

-

-

The PLS code shall be encoded according to clause 5.5.2.4.

In case the MSB b0 = 0, the result of header encoding according to clause 5.5.2.4 shall be identical to the
original DVB-S2 encoding applied to the 7 bits (b1, …, b7), and the interpretation of the 7 bits,
(b1,b2,…,b7), shall also be identical to the interpretation given in ETSI EN 302 307-1 [3], clause 5.5.2:
(b1, …, b5) shall represent the MODCOD field according to ETSI EN 302 307-1 [3], clause 5.5.2.2 and
ETSI EN 302 307-1 [3], Table 12, and the bits (b6, b7) shall represent the TYPE field according to ETSI
EN 302 307-1 [3], clause 5.5.2.3, i.e. (b6) shall indicate the frame length normal/short and (b7) the
presence/absence of pilots.

In case the MSB b0 = 1, (b1,b2,…,b6) shall represent the additional S2X MODCODs and the
corresponding FEC length (normal, short or medium) according to clause 5.5.2.2, while (b7) shall
indicate the presence/absence of pilots.

The entire PLHEADER (including SOF), represented by the binary sequence (y1, y2,...,y90) shall be modulated
into 90 π/2BPSK symbols according to the rule:

I2i-1 = Q2i-1 = (1/ 2 ) (1-2y2i-1), I2i = - Q2i = - (1/ 2 ) (1-2y2i) for i = 1, 2, ..., 13

If b0 = 0:

I2i-1 = Q2i-1 = (1/ 2 ) (1-2y2i-1), I2i = - Q2i = - (1/ 2 ) (1-2y2i) for i = 14, 15, ..., 45

If b0 = 1:

I2i-1 = - Q2i-1 = - (1/ 2 ) (1-2y2i-1), I2i = Q2i = - (1/ 2 ) (1-2y2i) for i = 14, 15, ..., 45

ETSI

46

ETSI EN 302 307-2 V1.4.1 (2024-08)

NOTE:

b0 = 0 the π/2BPSK modulation regularly continues after the SOF field as for S2, while if b0 = 1 a phase
jump of π/2 is introduced after the SOF field.

In case of Time slicing mode, PL signalling shall be according to ETSI EN 302 307-1 [3], Annex M.

5.5.2.1

SOF field

See ETSI EN 302 307-1 [3], clause 5.5.2.1.

5.5.2.2

MODCOD field

If b0 = 0, then (b1, b2, …, b5) shall be encoded according to ETSI EN 302 307-1 [3], clause 5.5.2.2 and ETSI
EN 302 307-1 [3], Table 12.

If b0 = 1, then (b1, b2, …, b6) shall be encoded according to Table 17a. PLS code decimal value is derived from
(b0, b1, b2, …, b7) with b0 = 1 and b7 = 0.

PLS code decimal value

Canonical MODCOD
name

Implementation MODCOD name

Code Type

Table 17a: S2X MODCOD Coding

129

131

132
134
136
138
140
142
144
146
148
150
152
154
156
158
160
162
164
166
168
170
172
174
178
180
182
184
186
190
194
198
200
202
204
206
208
210
212
214

VL SNR set1
See Table 18a
VL SNR set2
See Table 18a

QPSK 13/45
QPSK 9/20
QPSK 11/20
2+4+2APSK 100/180
2+4+2APSK 104/180
8PSK 23/36
8PSK 25/36
8PSK 13/18
8+8APSK 90/180
8+8APSK 96/180
8+8APSK 100/180
4+12APSK 26/45
4+12APSK 3/5
8+8APSK 18/30
4+12APSK 28/45
4+12APSK 23/36
8+8APSK 20/30
4+12APSK 25/36
4+12APSK 13/18
4+12APSK 140/180
4+12APSK 154/180
4+12+16rbAPSK 2/3
4+8+4+16APSK 128/180
4+8+4+16APSK 132/180
4+8+4+16APSK 140/180
16+16+16+16APSK 128/180
4+12+20+28APSK 132/180
8+16+20+20APSK 7/9
8+16+20+20APSK 4/5
8+16+20+20APSK 5/6
128APSK 135/180
128APSK 140/180
256APSK 116/180
256APSK 20/30
256APSK 124/180
256APSK 128/180
256APSK 22/30
256APSK 135/180

QPSK 13/45
QPSK 9/20
QPSK 11/20
8APSK 5/9-L
8APSK 26/45-L
8PSK 23/36
8PSK 25/36
8PSK 13/18
16APSK 1/2-L
16APSK 8/15-L
16APSK 5/9-L
16APSK 26/45
16APSK 3/5
16APSK 3/5-L
16APSK 28/45
16APSK 23/36
16APSK 2/3-L
16APSK 25/36
16APSK 13/18
16APSK 7/9
16APSK 77/90
32APSK 2/3-L
32APSK 32/45
32APSK 11/15
32APSK 7/9
64APSK 32/45-L
64APSK 11/15
64APSK 7/9
64APSK 4/5
64APSK 5/6
128APSK 3/4
128APSK 7/9
256APSK 29/45-L
256APSK 2/3-L
256APSK 31/45-L
256APSK 32/45
256APSK 11/15-L
256APSK 3/4

ETSI

Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal

PLS code decimal value

216
218
220
222
224
226
228
230
232
234
236
238
240
242
244
246
248

Canonical MODCOD
name
QPSK 11/45
QPSK 4/15
QPSK 14/45
QPSK 7/15
QPSK 8/15
QPSK 32/45
8PSK 7/15
8PSK 8/15
8PSK 26/45
8PSK 32/45
16APSK 7/15
16APSK 8/15
16APSK 26/45
16APSK 3/5
16APSK 32/45
32APSK 2/3
32APSK 32/45

47

ETSI EN 302 307-2 V1.4.1 (2024-08)

Implementation MODCOD name

Code Type

QPSK 11/45
QPSK 4/15
QPSK 14/45
QPSK 7/15
QPSK 8/15
QPSK 32/45
8PSK 7/15
8PSK 8/15
8PSK 26/45
8PSK 32/45
4+12APSK 7/15
4+12APSK 8/15
4+12APSK 26/45
4+12APSK 3/5
4+12APSK 32/45
4+12+16rbAPSK 2/3
4+12+16rbAPSK 32/45

Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short

Note that the PLS values in the table above correspond to the 'pilots off' case (b7 = 0), except for VL SNR sets with
pilots always on. Each MODCOD also has a 'pilots on' equivalent PLS code (b7 = 1). There are 16 additional PLS
sequences reserved for future use, but with a fixed frame-length associated to them, according to Table 17b.

Table 17b: S2X MODCOD Coding (Reserved values)

PLS code decimal value
128
130
176
177
188
189
192
193
196
197
250
251
252
253
254
255
n-ary is a generic denomination for any n-point constellation, to be defined in the future.

Mod and type
8-ary-normal-pilots off
16-ary - normal - pilots off
32-ary - normal - pilots off
32-ary - normal - pilots on
64-ary - normal - pilots off
64-ary - normal - pilots on
64-ary - normal - pilots off
64-ary - normal - pilots on
64-ary - normal - pilots off
64-ary - normal - pilots on
8-ary - normal - pilots on
16-ary - normal - pilots on
32-ary - normal - pilots on
64-ary - normal - pilots on
256-ary - normal - pilots on
1 024-ary - normal - pilots on

Length (symbols)
21 690
16 290
13 050
13 338
10 890
11 142
10 890
11 142
10 890
11 142
22 194
16 686
13 338
11 142
8 370
6 714

NOTE:

Note that these PLS codes are reserved but the S2X receiver should recognize these PLS codes and use the associated
frame-length in order to maintain lock (when confronted with one of these PLS codes). Note also that the pilot bit (b7)
does not indicate the presence of pilots for the last 6 PLS codes.

ETSI

48

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 18a: Definition of VL-SNR MODCODs

VL SNR set 1 (30 780 modulated symbols)

Code type
normal
medium
medium
medium
short
short

Code type
short
short
short

Canonical MODCOD name
QPSK 2/9
BPSK 1/5
BPSK 11/45
BPSK 1/3
BPSK-S 1/5
BPSK-S 11/45

π

π

Implementation MODCOD name
QPSK 2/9
/2 BPSK 1/5
/2 BPSK 11/45
π
/2 BPSK 1/3
/2 BPSK 1/5 Spreading Factor 2
/2 BPSK 11/45 Spreading Factor 2

π

π

VL SNR set 2 (14 976 modulated symbols)

Canonical MODCOD name
BPSK 1/5
BPSK 4/15
BPSK 1/3

Implementation MODCOD name
π
/2 BPSK 1/5
π
/2 BPSK 4/15
π
/2 BPSK 1/3

5.5.2.3

TYPE field

If b0 = 0, then (b6, b7) shall be coded according to ETSI EN 302 307-1 [3], clause 5.5.2.3.

If b0 = 1, then (b7) shall be coded according to ETSI EN 302 307-1 [3], clause 5.5.2.3.

5.5.2.4

PLS code, no time slicing

See ETSI EN 302 307-1 [3], clause 5.5.2.4.

The 8-bit header field shall be coded with a (64,8) code. Such code is constructed starting from a (32,7) code according
to the construction in Figure 19.

NOTE:

The symbol ⊗ stands for binary EXOR.

Figure 19

NOTE 1:  The particular construction guarantees that each odd bit in the (64,8) code is either always equal to the
previous one or is always the opposite. Which of the two hypotheses is true depends on the bit b7. This
fact can be exploited in case differentially coherent detection is adopted in the receiver.

The 7 most significant bits (b0, …, b6) of the header field shall be encoded by a linear block code of length 32 with the
following generator matrix.

ETSI

49

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure 20

NOTE 2:  Except from the inclusion of first row, the generator matrix corresponds to that of the S2 specification in
ETSI EN 302 307-1 [3], clause 5.5.2.4, and ETSI EN 302 307-1 [3], Figure 13b, and this guarantees the
correspondence of the PLS code for b0 = 0.

The most significant bit of the 8 bit header field is multiplied with the first row of the matrix, the following bit with the
second row and so on. The 32 coded bits is denoted as
(
(

. When b7 = 0, the final PLS code will generate
 as the output, i.e. each symbol shall be repeated. When b7 = 1, the final PLS code will generate
 as output, i.e. the repeated symbol is further binary complemented (see also Figure 6).

L
L

yy
21

L

)
)

32

32

(

)

y

yyyy
2211
yyyy
2211

yy
32
yy
32

32

The 64 bits output of the PLS code shall be further scrambled by the binary sequence:

0111000110011101100000111100100101010011010000100010110111111010.

In case of Time slicing the PLS code shall be according to Annex M.

The resulting 154 coded bits shall be scrambled with the following sequence:

1 0 1 1 1 1 0 0 0 0 0 1 0 0 0 1 1 0 0 0 0 0 0 0 1 0 1 0 0 1 0 1 0 1 0 1 0 0 0 1 0 1 1 1 0 1 1 1 0 1 0 1 1 1 0 0 1 0 0 1 1 1
0 1 1 0 0 0 0 1 0 1 1 0 0 0 1 1 1 1 1 1 0 1 1 0 1 0 1 1 0 0 1 1 0 1 1 0 1 1 1 0 0 0 0 1 1 1 0 0 0 1 1 0 1 0 1 0 0 1 1 1 1 1
0 0 0 1 0 0 0 0 1 1 0 0 1 0 1 0 0 0 0 0 0 1 1 1 1 0 1 1 1 1

5.5.2.5

VL-SNR Header

π

VL-SNR Headers shall be composed of LVL-SNR = 900 modulated symbols, the modulation format being

/2 BPSK.

Ten (10) such headers are currently defined. Six (6) other headers are currently unused. These headers shall be
constructed with a 896-bit sequence which arranged in the 16 56-bit rows below, from left to right, and top row to
bottom row, as shown below:

1111 1011 1111 0010 0011 1110 1000 0011 0111 1111 1001 1011 1100 0100
1001 1000 0111 0000 1000 1110 0000 1011 0011 1001 0011 0100 0101 1110
1111 0110 1010 0010 1100 1001 1111 1110 0001 1011 0001 0111 0011 0111
1000 0100 0001 1000 1101 1001 0101 1010 0110 1111 1001 1001 0111 1010
0111 1011 0111 1101 0111 1011 0011 1110 1001 1111 1100 1001 1110 1010
0101 1110 0111 1000 1011 1010 0000 0011 1010 0110 1101 0101 0001 1010
0010 0111 1001 1100 1100 0010 0110 0101 0100 0011 1110 1100 1101 0000
0011 0100 0010 1011 0000 0100 1001 1000 1011 1111 0011 1101 0111 1101
1010 1101 1101 0000 0011 0110 1110 1001 1101 0101 0011 0001 0010 1111
0001 0000 0110 0001 1100 0110 1101 1111 1000 0010 0110 0010 0011 0111
0111 0010 1101 0011 1110 0000 1001 0000 0111 0011 1000 0100 1100 0111
0011 1011 1101 0101 1010 1100 1110 1110 0010 0101 1110 0010 1100 1001
0101 1001 0000 1000 0111 1101 1000 0010 0110 0001 0101 1010 1101 1010
1110 1001 1010 1111 0000 0001 0111 0010 1100 1111 1001 1101 1010 0111
0011 1111 0100 1000 0011 0101 1010 0100 0000 0110 0011 1111 0000 0111
0010 0011 1100 1001 1010 1110 1110 1100 1111 0010 1110 1101 0100 0001

Sixteen (16) possible 896-bit patterns are constructed by multiplying each row with either + or - polarity according to
the 16 possible Walsh-Hadamard sequences below, where a "+" keeps the row unchanged, and a "-" changes every bit
in the row from a "0" to "1" and vice versa (Table 18b).

ETSI

50

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 18b: VL-SNR Header Walsh-Hadamard Sequence

Annex-I Index

VL SNR set 1 (30 780 modulated symbols), Acm=0xA0

Walsh-Hadamard Sequence

Implementation MODCOD name

Code type

0

1

2

3

4

5

6

7

8

9

10

11

12

13

14

++++++++++++++++

+_+_+_+_+_+_+_+_

++__++__++__++__

+__++__++__++__+

QPSK 2/9

π

/2 BPSK 1/5

π

/2 BPSK 11/45

π

/2 BPSK 1/3

normal

medium

medium

medium

++++____++++____

π

/2 BPSK 1/5 Spreading Factor 2

short

+__+_++_+__+_++_

π

/2 BPSK 11/45 Spreading Factor 2

short

++__++____++__++

+__++__+_++__++_

++++________++++

unassigned

unassigned

unassigned

VL SNR set 2 (14 976 modulated symbols) Acm=0xE0

Walsh-Hadamard Sequence

Implementation MODCOD name

Code type

++____++++____++

+_+__+_++_+__+_+

________++++++++

+_+_+_+__+_+_+_+

+_+__+_+_+_++_+_

++____++__++++__

+__+_++__++_+__+

short

short

short

N/A

π

/2 BPSK 1/5

π

/2 BPSK 4/15

π

/2 BPSK 1/3

dummy

unassigned

unassigned

unassigned

Each of the 896-bit pattern is padded at the beginning and the end with 00 to complete a 900 symbol-pattern.

5.5.2.6

Shortening and Puncturing of VL-SNR MODCODs

VL-SNR FECFRAMEs are defined in Tables 19a to 19d. A FECFRAME with nldpc = 32 400 bits has been included
covering only BPSK modulation and coding rates 1/5, 11/45, 1/3.

In order for VL-SNR frames to be compatible with legacy DVB-S2 VCM receivers, the PLFRAME length including
the mobile header and increased pilot symbols shall be the same as in DVB-S2 PLFRAME. This requires reducing the
information carrying symbols of VL-SNR frames through shortening and puncturing.

If an LDPC block is shortened, the first Xs information bits shall be set to zero before encoding, and they will not be
transmitted. If an LDPC block is punctured, every Pth parity bit starting with the first parity bit, p0, (i.e. p0, pP, p2P, …)
will not be transmitted until the desired number of punctured bits, Xp, is achieved.

ETSI

51

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table 19a: Shortening/Puncturing of VL-SNR FECFRAME

Implementation MODCOD name

π

π

QPSK 2/9 normal
/2 BPSK 1/5 medium
/2 BPSK 11/45 medium
π
/2 BPSK 1/3 medium
π
/2 BPSK 1/5 short SF2
/2 BPSK 11/45 short SF2
π
/2 BPSK 1/5 short
π
/2 BPSK 4/15 short
π
/2 BPSK 1/3 short

π

Xs
0
640
0
0
560
0
0
0
0

P

15
25
15
13
30
15
10
8
8

Xp
3 240
980
1 620
1 620
250
810
1 224
1 224
1 224

Table 19b: Coding Parameters for VL-SNR PLFRAMES (for normal FECFRAME nldpc = 64 800)

LDPC Code
Identifier

2/9

BCH uncoded block
Kbch
14 208

BCH coded block Nbch
LDPC uncoded block kldpc
14 400

BCH t-error
correction

12

LDPC coded block
nldpc
61 560

Table 19c: Coding Parameters for VL-SNR PLFRAMES (for medium FECFRAME nldpc = 32 400)

LDPC Code
Identifier

1/5
11/45
1/3

BCH uncoded block
Kbch
5 660
7 740
10 620

BCH coded block Nbch
LDPC uncoded block kldpc
5 840
7 920
10 800

BCH t-error
correction

12
12
12

LDPC coded block
nldpc
30 780
30 780
30 780

Table 19d: Coding Parameters for VL-SNR PLFRAMES (for short FECFRAME nldpc = 16 200)

LDPC Code
Identifier

11/45
4/15
1/3
1/5
1/5 SF2

BCH uncoded block
Kbch
3 792
4 152
5 232
3 072
2 512

BCH coded block Nbch
LDPC uncoded block kldpc
3 960
4 320
5 400
3 240
2 680

BCH t-error
correction

12
12
12
12
12

LDPC coded block
nldpc
15 390
14 976
14 976
14 976
15 390

5.5.3

Pilot Insertion

See ETSI EN 302 307-1 [3], clause 5.5.3.

5.5.4

Physical layer scrambling

5.5.4.0

General aspects

See ETSI EN 302 307-1 [3], clause 5.5.4.

While ETSI EN 302 307-1 [3], clause 5.5.4 declares: "In case of broadcasting services, n = 0 shall be used as default
sequence, to avoid manual receiver setting or synchronization delays", in order to mitigate interference in a satellite
system, 6 additional different scrambling code sequences may be used in S2X also for the broadcast application when
pilots are inserted in the PLFRAME (b7 = 1, see clause 5.5.2.3).

For all relevant S2X applications using different PL-scrambling sequences, to facilitate initial acquisition in the absence
of side information, a shortlist of 7 preferred scrambling code sequences with good mutual interference properties is
defined in Table 19e. All frames in a carrier shall be scrambled using the same scrambling sequence.

ETSI

52

ETSI EN 302 307-2 V1.4.1 (2024-08)

NOTE 1:  In case of sequential initial acquisition in the receiver, the first scrambling code sequence (n = 0) is tested

first.

NOTE 2:  Any other scrambling sequence can be used; the demodulator should be informed about the scrambling

sequences to be used (e.g. through network signalling information, or by having them stored in the
demodulator).

Table 19e: Set of preferred scrambling sequences

Scrambling sequence
0
1
2
3
4
5
6

Gold sequence index n
0
10 949
2 x 10 949
3 x 10 949
4 x 10 949
5 x 10 949
6 x 10 949

5.5.4.1

PL scrambling for VL-SNR frames

5.5.4.1.0

General aspects

VL-SNR frames shall not scramble PLHEADERs and shall not scramble VL-SNR-HEADER.

symbols

990

PLS Header 1 slot

VL SNR Header 10 slots

Pilots

FEC Frame

…

…

Scrambling Sequence Inactive

Scrambling Sequence Active

Scrambling
Reset

(Scrambled)PL Frame

Figure 21: PL SCRAMBLING

For VLNSR frames, the randomization sequence shall be reinitialized at the end of the PLS Header and shall remain
inactive during VL SNR Header.

5.5.4.1.1

π/2-BPSK modulated frames

For π/2-BPSK modulated XFECFRAMEs (see Table 18a, VL-SNR), the 2-valued multiplication factor (CI+jCQ) shall
be used for Physical layer scrambling (instead of the 4-valued multiplication factor (CI+jCQ) defined in ETSI
EN 302 307-1 [3], clause 5.5.4):

CI(i) + jCQ(i) = exp (j Rn (i) π)

Pilot symbols and VL-SNR dummy symbols shall be scrambled using the factor (CI+jCQ) defined in ETSI
EN 302 307-1 [3], clause 5.5.4.

ETSI

53

ETSI EN 302 307-2 V1.4.1 (2024-08)

5.6

Baseband shaping and quadrature modulation

See ETSI EN 302 307-1 [3], clause 5.6.

In addition to the S2 roll-off factors (α = 0,35, 0,25 and 0,20), the additional roll-offs α = 0,15; 0,10 and 0,05 shall be
implemented.

6

Error performance

Tables 20a to 20c summarize the S2X modes performance requirements at QEF over AWGN and Hard Limiter (see
Figure H.2 in clause H.7) channels. Ideal performance figures have been achieved by computer simulation, 50 LDPC
fixed point decoding iterations, perfect carrier and synchronization recovery, no phase noise. For calculating link
budgets, specific satellite channel impairments should be taken into account.

FER is the ratio between the useful FECFRAMEs correctly received and those affected by errors, after forward error
correction.

Table 20a: Performance at Quasi Error Free FER=10-5
Normal FECFRAMES, 50 iterations

Canonical MODCOD
name

Spectral efficiency
[bit/symbol]
(note 4)

Ideal Es/N0 [dB] for
(AWGN Linear Channel)
(note 1)

QPSK 2/9
QPSK 13/45
QPSK 9/20
QPSK 11/20
8APSK 5/9-L
8APSK 26/45-L
8PSK 23/36
8PSK 25/36
8PSK 13/18
16APSK 1/2-L
16APSK 8/15-L
16APSK 5/9-L
16APSK 26/45
16APSK 3/5
16APSK 3/5-L
16APSK 28/45
16APSK 23/36
16APSK 2/3-L
16APSK 25/36
16APSK 13/18
16APSK 7/9
16APSK 77/90
32APSK 2/3-L
32APSK 32/45
32APSK 11/15
32APSK 7/9
64APSK 32/45-L
64APSK 11/15
64APSK 7/9
64APSK 4/5
64APSK 5/6
128APSK 3/4
128APSK 7/9
256APSK 29/45-L
256APSK 2/3-L
256APSK 31/45-L
256APSK 32/45

0,434841
0,567805
0,889135
1,088581
1,647211
1,713601
1,896173
2,062148
2,145136
1,972253
2,104850
2,193247
2,281645
2,370043
2,370043
2,458441
2,524739
2,635236
2,745734
2,856231
3,077225
3,386618
3,291954
3,510192
3,620536
3,841226
4,206428
4,338659
4,603122
4,735354
4,936639
5,163248
5,355556
5,065690
5,241514
5,417338
5,593162

-2,85 (note 3)
-2,03
0,22
1,45
4,73
5,13
6,12
7,02
7,49
5,97
6,55
6,84
7,51
7,80
7,41
8,10
8,38
8,43
9,27
9,71
10,65
11,99
11,10
11,75
12,17
13,05
13,98
14,81
15,47
15,87
16,55
17,73
18,53
16,98
17,24
18,10
18,59

ETSI

Ideal Csat/(N0

⋅Rs) [dB]

(Non-Linear Hard Limiter
Channel)
(informative)
(note 2)
-2,45
-1,60
0,69
1,97
5,95
6,35
6,96
7,93
8,42
8,4
9,0
9,35
9,17
9,38
9,94
9,76
10,04
11,06
11,04
11,52
12,50
14,00
13,81
14,50
14,91
15,84
17,7
17,97
19,10
19,54
20,44
21,43
22,21
21,6
21,89
22,9
22,91

54

ETSI EN 302 307-2 V1.4.1 (2024-08)

Canonical MODCOD
name

Spectral efficiency
[bit/symbol]
(note 4)

Ideal Es/N0 [dB] for
(AWGN Linear Channel)
(note 1)

256APSK 11/15-L
256APSK 3/4

5,768987
5,900855

18,84
19,57

Ideal Csat/(N0

⋅Rs) [dB]

(Non-Linear Hard Limiter
Channel)
(informative)
(note 2)
23,80
24,02

NOTE 1:  Es is the average energy per transmitted symbol; N0 is the noise power spectral density.
NOTE 2:  Csat is the Hard Limiter pure carrier saturated power; N0

⋅Rs is the Noise Power integrated over a bandwidth

equal to the symbol rate. Performance results are for an optimized input back-off (IBO) and for a
⋅Rs) is equal to Es,sat/N0 and the difference between the Es/N0 of the AWGN linear
Roll-off=10 %. Csat/ (N0
channel and Es,sat /N0 is due to the compromise between operating back-off and nonlinear distortion (which
is dependent on the rolloff).

NOTE 3:  The FECFRAME length is 61 560.
NOTE 4:  Spectral efficiencies are calculated in a bandwidth equal to the symbol rate Rs in case of no pilots. The
corresponding spectral efficiency for a bandwidth equal to Rs (1+roll-off) can be computed dividing the
numbers in column "spectral efficiency" by (1+roll-off).

Table 20b: Es/N0 Performance at Quasi Error Free FER=10-5 (AWGN Channel)
medium XFECFRAMEs, 75 iterations

Canonical MODCOD name

BPSK 1/5
BPSK 11/45
BPSK 1/3

Ideal Es/N0 (dB) for
FECFRAME length = 30 780
-6,85
-5,50
-4,00

Table 20c: Es/N0 Performance at Quasi Error Free FER=10-5 (AWGN Channel)
/2 BPSK modes, 50 iterations other modes
Short XFECFRAMEs, 75 iterations

π

Canonical MODCOD name

BPSK-S 1/5
BPSK-S 11/45
BPSK 1/5
BPSK 4/15
BPSK 1/3
QPSK 11/45
QPSK 4/15
QPSK 14/45
QPSK 7/15
QPSK 8/15
QPSK 32/45
8PSK 7/15
8PSK 8/15
8PSK 26/45
8PSK 32/45
16APSK 7/15
16APSK 8/15
16APSK 26/45
16APSK 3/5
16APSK 32/45
32APSK 2/3
32APSK 32/45

Ideal Es/N0 (dB) for
FECFRAME length = 16 200
-9,9 (note 1)
-8,3 (note 1)
-6,1 (note 2)
-4,9 (note 2)
-3,72
-2,50
-2,24
-1,46
0,60
1,45
3,66
3,83
4,71
5,52
7,54
5,99
6,93
7,66
8,10
9,81
11,41
12,18

NOTE 1:  The FECFRAME length is 15 390.
NOTE 2:  The FECFRAME length is 14 976.

ETSI

55

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex A (normative):
Signal spectrum at the modulator output

See ETSI EN 302 307-1 [3], Annex A.

Figure A.1 gives a template for the signal spectrum at the modulator output.

Figure A.1 also represents a possible mask for a hardware implementation of the Nyquist modulator filter. The points
A to S shown on Figures A.1 and A.2 are defined in Table A.1. The mask for the filter frequency response is based on
the assumption of ideal Dirac delta input signals, spaced by the symbol period TS = 1/RS = 1/2fN while in the case of
rectangular input signals a suitable x/sin x correction shall be applied on the filter response.

Relative power (dB)

10

0

-10

-20

-30

-40

-50

A

B

C

D

E

F

G

I

J

H

K

L

M

P

Q

N

S

0

0,5

1

1,5

2

2,5

3

f/f N

Figure A.1: Template for the signal spectrum mask at the modulator output represented in the
baseband frequency domain, the frequency axis is calibrated for roll-off factor α = 0,35

Figure A.2 gives a mask for the group delay for the hardware implementation of the Nyquist modulator filter.

Group delay  x  f    N

0,2

0,15

0,1

0,05

A

C

E

G

I

0,00
B

D

0,50

F

H

0

-0,05

-0,1

-0,15

-0,2

J

1,00

K

L

M

1,50

2,00

2,50

3,00

f / f

N

Figure A.2: Template of the modulator filter group delay

ETSI

56

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table A.1: Definition of points given in Figures A.1 and A.2 (see note)

Point

A

B

C

D

E

F

G

H

I

J

K

L

M

N

P

Q

S

Frequency
for α = 0,15
0,0 fN
0,0 fN
0,2 fN
0,2 fN
0,4 fN
0,4 fN
0,9175 fN
0,9175 fN
0,955 fN
1,0 fN
1,0 fN
1,0825 fN
1,0825 fN
1,375 fN
1,1725 fN
1,3 fN
1,525 fN

Frequency
for α = 0,10
0,0 fN
0,0 fN
0,2 fN
0,2 fN
0,4 fN
0,4 fN
0,945 fN
0,945 fN
0,97 fN
1,0 fN
1,0 fN
1,055 fN
1,055 fN
1,25 fN
1,115 fN
1,2 fN
1,35 fN

Frequency
for α = 0,05
0,0 fN
0,0 fN
0,2 fN
0,2 fN
0,4 fN
0,4 fN
0,9725 fN
0,9725 fN
0,985 fN
1,0 fN
1,0 fN
1,0275 fN
1,0275 fN
1,125 fN
1,0575 fN
1,1 fN
1,175 fN

Relative power
(dB)
+0,25

-0,25

+0,25

-0,40

+0,25

-0,40

+0,15

-1,10

-0,50

-2,00

-4,00

-8,00

-11,00

-35,00

-16,00

-24,00

-40,00

Group delay

+0,07/fN
-0,07/fN
+0,07/fN
-0,07/fN
+0,07/fN
-0,07/fN
+0,07/fN
-0,07/fN
+0,07/fN
+0,07/fN
-0,07/fN
-

-

-

-

-

-

NOTE:

See ETSI EN 302 307-1 [3], Annex A for roll-off α = 0,35, 0,25 and 0,20.

ETSI

57

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex B (normative):
Addresses of parity bit accumulators for nldpc = 64 800

Table B.1: LDPC code identifier: 2/9 (nldpc = 64 800)

5332 8018 35444 13098 9655 41945 44273 22741 9371 8727 43219
41410 43593 14611 46707 16041 1459 29246 12748 32996 676 46909
9340 35072 35640 17537 10512 44339 30965 25175 9918 21079 29835
3332 12088 47966 25168 50180 42842 40914 46726 17073 41812 34356
15159 2209 7971 22590 20020 27567 4853 10294 38839 15314 49808
20936 14497 23365 22630 38728 28361 34659 956 8559 44957 22222
28043 4641 25208 47039 30612 25796 14661 44139 27335 12884 6980
32584 33453 1867 20185 36106 30357 809 28513 46045 27862 4802
43744 13375 36066 23604 30766 6233 45051 23660 20815 19525 25207
27522 3854 9311 21925 41107 25773 26323 24237 24344 46187 44503
10256 20038 12177 26635 5214 14191 34404 45807 4938 4173 31344
32043 26501 46725 4648 16718 31060 26633 19036 14222 13886 26535
18103 8498 36814 34600 36495 36712 29833 27396 11877 42861 1834
36592 1645 3649 30521 14674 3630 890 13307 41412 24682 9907
4401 44543 13784 5828 32862 25179 29736 39614 5186 49749 38317
41460 39101 50080 40137 32691 26528 35332 44067 8467 14286 10470
12211 34019 37870 36918 36419 33153 50070 41498 47741 30538 12342
33751 23988 33624 41882 34075 25552 3106 17611 13190 29336 312
5667 35483 35460 16153 37267 28308 50009 46345 34204 32756 38243
5657 24157 36834 6890 49576 46244 43875 16738 47225 2944 36882
30341 48485 3700
14451 20438 18875
13634 41138 42962
46459 13369 27974
21493 14629 2369
11351 40226 42457
34749 39000 3912
18128 46776 47055
2221 26806 11345
35143 630 2229
44009 41295 34646
32163 16657 26544
31770 23641 43623
45826 10902 39490
7514 20480 28511
11429 19834 35430
50112 38163 5738
16191 16862 6783
6085 39149 34988
41497 32023 28688

Table B.2: LDPC code identifier: 13/45 (nldpc = 64 800)

15210 4519 18217 34427 18474 16813 28246 17687 44527 31465 13004 43601
28576 13611 24294 15041 503 11393 26290 9278 19484 20742 13226 28322
32651 27323 22368 15522 37576 20607 20152 19741 26700 31696 21061 35991
44168 27910 31104 34776 38835 45450 40002 31522 7807 26330 2410 44983
15861 39215 14631 42584 26502 41864 27885 32276 29049 16878 37480 42550
38795 13012 7912 4058 23869 3325 42889 19921 13826 40323 18162 10005
35100 5483 7629 35166 1239 10772 5289 286 16172 41843 42612 38493
11997 40340 19047 16236 43557 9104 24032 2915 19265 36209 6443 40947
43527 29675 4195 31926 35392 20400 7515 45806 36068 33079 37325 6301
4580 20492 40934 14478 8238 2425 28901 43602 7224 17640 28259 6850

ETSI

58

ETSI EN 302 307-2 V1.4.1 (2024-08)

41859 14006 19132 5690 16223 11575 30562 44797 3759 9833 36529 21084
45546 16044 26763 13559 29092 41595 5726 13733 9164 15354 20145 10655
24076 40883 13424 30325 40589 32367 36270 9286 40151 8501 3871 22109
26239 29805 5358 44835 11609 3899 9760 39600 43422 13295 45431 14515
5392 37010 12386 40193 21492 45146 12376 41952 43153 45733 718 35726
33884 38006 16927 20958 25413 44561 11245 12984 35198 30977 31916 10657
1412 1048 14965 31879 29967 41000 32087 22 34773 768 27289 19898
43051 6964 31807 4119 33509 15950 6304 2813 35192 38282 39710 26356
9889 18957 6355 18770 40381 1876 38889 17958 20309 10744 1744 228
41543 36505 32795 12454 8520 4916 22313 1363 13010 8770 17057 8694
22987 29564 13804 3110 1382 33844 15117 42314 36045 25295 28421 22044
15951 42952 17458 6926 21257 41243 8662 17046 15054 15302 16964 40079
13359 45754 16715 9586 10960 25406 14675 8880 5087 12303 28993 13571
24824 31012 4121 808 30962 28736 11013 20488 7715 7637 6217 25114
23615 5760 5554
18072 21605 39242
24190 6592 12281
44681 6563 7001
18291 19605 33476
2884 30927 18430
23674 36414 30649
15364 22089 19757
41162 14454 17627
16676 28573 22163
8851 36803 27589
40049 476 1413
41013 34505 33296
29782 38018 42124
22625 7485 11772
2052 37567 14082
30106 43203 20858
7399 3796 22396
38745 792 44483
28268 33355 41030
30098 37269 12871
35769 33119 16738
3307 43434 13244
17852 9133 23190
35184 20115 24202
14760 43026 19425
26414 16821 6625
30362 35769 42608

Table B.3: LDPC code identifier: 9/20 (nldpc = 64 800)

30649 35117 23181 15492 2367 31230 9368 13541 6608 23384 18300 5905
1961 8950 20589 17688 9641 1877 4937 15293 24864 14876 6516 10165
4229 26034 28862 8265 27847 3 22728 13946 27162 26003 17696 13261
31719 25669 17149 17377 33106 12630 4814 16334 1480 32952 11187 3849
30186 20938 7946 23283 11042 28080 26642 34560 11302 4991 5121 6879
13445 22794 18048 15116 5657 9853 15581 34960 13240 11176 17937 25081
4868 28235 30286 29706 7073 6773 10390 27002 13015 7388 14772 19581
11765 16642 11431 19588 20154 8027 29758 5501 6398 4268 21337 21136
2275 7899 25943 12939 14478 20369 22877 3591 12217 19130 24252 32444
24599 21382 4689 3524 11304 20423 13677 19639 10577 28279 22330 30722
21622 26233 3921 17722 6843 5999 8186 2355 33632 34632 30285 9616
19909 30417 19587 27853 13896 3689 155 20457 33362 21739 22779 33862
3713 32975 9403 2836 23109 11099 3505 14562 17309 26470 4843 12279
24216 26340 22073 32570 12936 19797 21801 8918 7999 24408 5783 25190
8817 29367 17017 6208 21402 2280 2110 7975 32039 34605 1235 912
23116 33017 31405 638 4707 31760 18043 3507 11989 26632 32829 11262

ETSI

59

ETSI EN 302 307-2 V1.4.1 (2024-08)

9274 2553 10697 13507 15323 27080 3752 33191 12363 24664 14068 1416
21670 26696 18570 25197 1517 7765 32686 6572 30901 28242 17802 24056
35388 26895 8023 31249 29290 13440 7156 17367 21472 27219 14447 9655
11100 27918 2900 33262 15301 4664 15728 1185 24818 32995 31108 16368
34978 31690 30464 13044 5492 10047 2768 14336 30880 32780 10993 24750
7022 19718 26036 19145 21177 33949 17135 5193 33718 2539 13920 25537
918 18514 14530 13699 11902 22721 8335 35346 24655 3332 14708 20822
11191 24064 32825 12321 11771 23299 31325 25526 16785 22212 34075 9066
31209 27819 5974 19918 26831 33338 26647 9480 28489 7827 18562 2401
17395 23192 10277 28458 23028 18793 10463 10740 616 24647 4153 10128
2873 22381 8132 18239 31614 4193 32313 7575 25801 27591 19872 17992
4609 9114 14764 13516
19192 9882 13112 16075
12510 28902 8784 32679
4578 34533 30609 25543
13739 3465 5330 999
33254 13085 5001 29061
28369 79 17750 13399
24851 9524 30966 10422
18251 34810 12259 25103
25193 16945 1059
11266 13612 30508
24778 25364 1322
14492 11111 13693
15125 8205 1749
8494 9902 9395
23936 3981 22799
28448 28076 26544
19652 13424 8915
2885 11356 3241
1609 10284 24350
2462 19358 15717
29327 15960 14743
5388 32927 1288
19074 6322 32214
34208 30535 35462
23415 20836 21819
17986 12196 30030
8422 2647 5710
3200 23132 23337
22307 29841 4813
15309 26942 29970
23288 7493 3005
20661 34283 33192
23033 9541 6424
22003 24665 5534
4684 1411 33340
26042 6426 3808
285 21942 14302
16023 6825 20084
34878 12295 32028
2591 178 24107
16379 2912 9912
15375 16120 28375
20170 726 11291
8185 13471 8448
23205 14239 17896
17950 19308 1591
3170 23836 18879
12853 10678 18431
21157 31624 3153
27682 12433 3458

ETSI

60

ETSI EN 302 307-2 V1.4.1 (2024-08)

312 4844 13138
17715 35138 15456
30507 33307 30783

Table B.4: LDPC code identifier: 11/20 (nldpc = 64 800)

20834 22335 21330 11913 6036 15830 11069 10539 4244 15068 7113 2704 16224
2010 5628 27960 11690 22545 24432 4986 21083 17529 4104 11941 21239 9602
689 13248 1777 4876 2537 20869 15718 9575 18164 5294 13914 21711 23374
9675 21239 13600 24710 10613 14804 19412 23270 26741 10503 25258 17816 25210
12518 8680 6422 22715 25097 26959 3913 26493 7797 25977 4896 27063 20781
21715 12850 7963 4027 4295 14931 18158 616 20570 8720 16487 19050 23925
7939 21089 15170 24325 6651 22352 5633 27903 2685 1310 5594 9296 25670
25121 13906 8217 25390 9112 13945 9826 10844 11418 10724 11518 9280 9576
25979 23644 16073 27407 3476 28057 4003 2279 17490 7558 9538 22115 20439
20708 22572 14997 15780 5159 11356 10931 8514 23275 2560 912 15935 20703
26467 17173 21964 15469 21967 10380 16222 15106 16786 19542 28560 18387 27909
14897 6167 24295 1266 16902 9546 11628 12048 24495 3706 22629 14165 2333
19403 18738 28140 13141 6151 22785 9620 4290 2342 4902 15856 19033 22820
15761 1985 9160 4435 11164 5442 23572 6951 19077 15406 16658 18324 19229
16997 10094 19982 22821 7810 19660 1182 21968 16564 17453 10780 17034 16405
11 28611 10411 15799 15705 2773 28601 19333 19447 16790 4618 15841 23854
24686 4131 1013 2141 6052 11896 18719 16813 22420 23406 21052 4333 17754
16425 17614 26883 12101 8224 13979 6869 25215 25991 28968 19337 25361 20513
1671 14990 20692 24951 19446 7163 4959 13197 19201 3883 22532 15468 11856
22758 23586 16985 18396 7434 11817 363 11824 285 20897 16646 16095 17011
25144 14916 6302 20972 25439 6156 21776 19701 27803 9695 12941 23541 27425
6979 27910 7378 8983 6280 4134 28860 8079 20892 28776 7899 23399 87
18045 23929 25876 15560 23629 18376 4053 14655 2450 11907 19535 28543 3513
4704 16512 16554 14062 2596 10357 17316 1011 22090 11353 20300 15300 18536
14293 4746 28831 20028 16742 16835 28405 11245 10802 20242 17737 9590 20693
26547 22557 22517 6285 5336 3998 2351 6628 22949 1517 4712 1770 9207
28522 14116 5455 13105 18709 3030 4217 6306 27448 1943 23866 20212 18857
14794 21425 15659
4446 21140 13454
21115 3271 1443
2153 12424 6159
23559 22473 26065
15914 22980 12766
3482 16233 5719
27020 12322 24014
25438 26499 26506
21987 16027 6832
17330 2620 20756
15985 10471 23302
593 6869 27185
22961 9129 25646
10702 12334 23959
6375 23299 26942
8029 4072 24051
15147 5113 14725
1451 27291 28731
18808 11561 249
28962 21405 18944
6889 3314 23457
27708 14530 8795
6185 28821 6550
2259 17627 701
20819 18831 20140
4991 11369 4282
13230 3413 27092

ETSI

61

ETSI EN 302 307-2 V1.4.1 (2024-08)

14556 5068 16209
4337 24652 498
715 28883 2285
16524 25513 26034
21067 15122 21667
27982 15280 3313
7563 22779 22453
4744 17277 27210
19170 10806 18815
26424 26442 7837
26264 28931 6020
4645 20678 13160
18111 28045 23883
5128 10876 3087
28551 26276 3541
20152 10181 28172
26430 14769 6809
4956 16130 11348
1691 10216 5743
7848 20236 2661
10660 8321 6155
2757 6963 2596
27791 6707 258
12785 21176 15450
7477 17274 25201
262 18996 15836
5287 11970 13365
3098 17823 10786
21831 14476 11447
1893 3625 25404
20880 21987 1228
20942 15045 21358
18237 28914 15673
24273 284 9803
13949 15670 16693
15553 27782 22644
27980 24820 27733
7015 20974 10016
26164 20314 25916
11489 13663 11777
18230 11483 5655
1618 19977 26521
25639 13184 28994
3821 18349 13846

Table B.5: LDPC code identifier: 26/45 (nldpc = 64 800)

12918 15296 894 10855 350 453 11966 1667 18720 12943 24437 8135 2834
11861 3827 15431 8827 8253 23393 15048 5554 16297 2994 6727 19453 2371
26414 3044 20240 18313 11618 3145 10976 5786 5609 16358 2547 11557 14755
26434 2510 26719 4420 6753 917 7821 26765 11684 9811 5420 6653 19554
11928 20579 17439 19103 21162 11235 19172 22254 3420 10558 3646 11858 24120
10189 8172 5004 26082 4345 5139 15135 26522 6172 17492 8462 4392 4546
27330 21498 13424 8077 10165 9739 482 23749 1515 12788 10464 9085 20875
12009 22276 18401 7541 5871 23053 16979 16300 13566 19424 5293 18290 23917
9613 24175 11374 11736 17676 13126 20931 20290 20659 2000 7969 9386
21507 24494 11822 21771 26776 21175 27354 15815 7598 19809 611 10144
195 14244 7229 13002 14328 17987 14595 6985 7642 9434 7079 5571
10013 3641 14064 11716 4620 18119 23365 26446 26273 25164 11262 26019
15166 19403 5606 20138 1893 645 5414 12097 18635 21648 12255 13269
1895 9969 8372 17737 21679 17061 20219 2513 27199 11242 17025 1261
12845 13086 16256 15177 20822 10862 18375 6751 17532 24725 6966 18489

ETSI

62

ETSI EN 302 307-2 V1.4.1 (2024-08)

8373 25550 20688 16686 7894 24599 21578 12516 7115 4836 23473 25162
14375 9150 6606 21633 16224 23708 20350 4575 143 13356 10239 22868
10760 19807 7079 16382 26236 22606 16777 24312 16941 26684 8658 19279
15136 8603 332 2898 21821 23778 3232 12052 14336 7832 5600 27015
14392 26564 21616 8332 21750 10379 19730 7553 27352 2718 15202 25661
6891 13210 15284 21940 8742 10965 3176 25034 25137 25161 13267 7012
4993 9943 13260 20980 20224 20129 2120 23111 16640 23548 21445 10794
4846 2858 22663 12584 20448 4629 17825 22269 11278 26312 9463 21085
24282 18233 9220 14979 24106 14507 24838 19689 17589 7926 7893 21701
12253 26122 8035 20823 2584 4703 25178 5460 4190 7057 1144 8426
12354 7216 19484 4110 22105 1452 11457 12539 27106 14256 14113 20701
2547 26926 25933 11919 12026 24639 19741 15457 9239 26713 22838 6051
8782 14714 23363 450 19972 2622 19473 24182 2391 26205 10018 9202
15690 10472 20263 469 18876 23660 9005 12595 23818 26430 926 6156
5440 5209 14958 9882 18843 22063 12749 18473 22546 11768 4493 12833
18540 3544 9471 15893 14761 23479 22010 15491 19608 25035 9094 24836
15909 16594 23538 25136 25063 24995 5354 905 18580 15476 20710 7774
6088 17133 11498
4721 17594 18267
1645 23638 26645
14800 17920 22016
12927 350 19391
19447 19886 25992
26120 1747 11234
1588 23170 27232
2230 15468 18709
17410 11055 20645
3244 25815 14204
2858 7980 12780
3256 20418 24355
24260 16245 20948
11122 1503 15651
19272 24054 6075
4905 931 18884
23633 17244 6067
5568 26403 490
16113 16055 10524
23013 8138 12876
20699 20123 15435
27272 27296 22638
7658 17259 20553
14914 17891 12137
16323 1085 18895
21503 17141 2915
21979 23246 1271
14409 11303 12604
25591 12157 14704
18739 19265 8140
11244 5962 6647
3589 6029 6489
16416 185 9426
1267 14086 22473
17159 22404 23608
7230 22514 21605
7645 1239 10717
12028 13404 12140
14784 15425 14895
26165 18980 15386
14399 7725 14908
8463 22853 22095
5517 1854 8283
24381 260 12595

ETSI

63

ETSI EN 302 307-2 V1.4.1 (2024-08)

839 23743 22445
13473 8017 7716
8697 13050 16975
26656 16911 11972
26173 2504 15216
7493 6461 12840
4464 14912 3745
21461 9734 25841
4659 7599 9984
17519 7389 75
12589 9862 8680
23053 21981 25299
19246 3243 15916
21733 4467 26491
4959 10093 20074
9140 15000 12783
854 10701 25850
13624 7755 10789
3977 15812 10783
5830 6774 10151
21375 25110 5830
15985 18342 2623
4716 27211 18500
18370 12487 7335
4362 21569 16881
10421 15454 13015
5794 1239 9934

Table B.6: LDPC code identifier: 28/45 (nldpc= 64 800)

24402 4786 12678 6376 23965 10003 15376 15164 21366 24252 3353
8189 3297 18493 17994 16296 11970 16168 15911 20683 11930 3119
22463 11744 13833 8279 21652 14679 23663 4389 15110 17254 17498
13616 426 18060 598 19615 9494 3987 8014 13361 4131 13185
4176 17725 14717 3414 10033 17879 8079 12107 10852 1375 19459
1450 4123 2111 17490 13209 8048 15285 4422 11667 18290 19621
2067 15982 304 8658 19120 6746 13569 19253 2227 22778 23826
11667 11145 20469 17485 13697 3712 4258 16831 22634 18035 7275
23804 14496 17938 15883 14984 15944 2816 22406 22111 2319 14731
8541 12579 22121 8602 16755 6704 23740 16151 20297 9633 1100
19569 10549 19086 21110 11659 6901 21295 7637 11756 8293 9071
9527 9135 7181 19534 2157 788 13347 17355 17509 711 20116
21217 15801 12175 9604 17521 2127 21103 1346 8921 7976 3363
11036 5152 19173 8086 3571 1955 4146 13309 15934 19132 5510
12935 13966 15399 16179 8206 19233 16702 7127 12185 15420 1383
6222 6384 20549 18914 23658 11189 638 9297 17741 9747 13598
17209 11974 20776 2146 9023 3192 19646 3393 1727 15588 20185
5008 3885 5035 15852 5189 13877 15177 3049 22164 16540 21064
24004 10345 12255 36 24008 8764 13276 13131 2358 24010 16203
21121 21691 8555 11918 129 8860 23600 3042 3949 19554 12319
22514 11709 11874 11656 536 9142 3901 580 1547 10749 5529
3324 6251 1156 112 13086 5373 5119 132 18069 10482 19519
17279 2017 14846 21417 17154 21735 18788 11759 192 16027 6234
20417 3788 15159 22188 21251 16633 13579 8128 1841 23554 15056
12104 9182 6147 1553 12750 4071 6495
4961 18460 23266 10785 10973 4405 2707
7665 7043 1968 3589 15378 9642 21148
13073 13298 20040 13582 17124 348 12055
378 7476 9838
15454 5218 14834
17678 3445 18453

ETSI

64

ETSI EN 302 307-2 V1.4.1 (2024-08)

2767 388 12638
5688 56 6360
20009 872 16872
10206 5551 477
10662 23689 19768
8965 17535 4421
19397 18734 5422
10043 22104 21682
508 1588 23853
1092 7288 4358
2283 22298 10504
15022 8592 22291
11844 17038 2983
17404 14541 6446
20724 7498 2993
14715 9410 6844
20213 14674 263
4822 20951 635
20651 23174 5057
22237 9229 4859
17280 9586 20334
19508 8068 11375
5776 21209 9418
6872 6349 20397
11165 19619 13108
13550 10715 5122
5655 10699 8415
9864 4985 7986
6436 3754 7690
4257 17119 5328
659 4687 6006
527 10824 8234
11291 1735 22513
7254 2617 1493
3015 7462 10953
15705 2181 11992
4628 19430 18223
9426 21808 13549
17008 3470 22568
13643 24195 21816
936 14226 22874
6156 19306 18215
23984 14714 12907
5139 18639 15609
11908 5446 8958
6315 16864 15814
10686 22570 16196
203 4208 13716
494 14172 11778
15112 14244 8417
21087 4602 15570
19758 4401 22270
8218 11940 5009
23833 13785 12569
1698 7113 18541
18711 19991 19673
8025 17107 14784
5954 6817 19810
24143 12236 18063
23748 23956 10369
7805 13982 13861
5198 10889 6787

ETSI

65

ETSI EN 302 307-2 V1.4.1 (2024-08)

10406 13918 3305
12219 6523 12999
9964 2004 17361
23759 21507 11984
4188 19754 13358
8027 3662 2411
19762 16017 9125
2393 4619 5452
24176 6586 10895
15872 1795 15801
6911 15300 14787
2584 4905 8833
1327 12862 9476
16768 12633 7400
11983 6276 18370
12939 12793 20048
20284 12949 21345
19545 4503 16017
1253 12068 18813

Table B.7: LDPC code identifier: 23/36 (nldpc = 64 800)

2475 3722 16456 6081 4483 19474 20555 10558 4351 4052 20066
1547 5612 22269 11685 23297 19891 18996 21694 7927 19412 15951
288 15139 7767 3059 1455 12056 12721 7938 19334 3233 5711
6664 7486 17133 2931 20176 20158 9634 20002 13129 10015 13595
218 22642 9357 11999 22898 4446 8059 1913 22365 10039 15203
10305 22970 7928 16564 8402 9988 7039 10195 22389 5451 8731
19073 1005 18826 11109 13748 11891 21530 15924 21128 6841 11064
3240 11632 18386 22456 3963 14719 4244 4599 8098 7599 12862
5666 11543 9276 19923 19171 19591 6005 8623 22777 1255 20078
17064 13244 323 11349 6637 8611 6695 4750 20985 18144 5584
20309 6210 16745 10959 14284 2893 20916 10985 9664 9065 11703
17833 21598 22375 12890 10779 11241 13115 9222 21139 1217 15337
15514 12517 18953 11458 17296 8751 7213 12078 4994 4391 14976
3842 21548 10955 11679 16551 8514 17999 20557 16497 12122 23056
10551 20186 66 11038 22049 2130 1089 22093 9069 3470 8079
19208 22044 2732 1325 22309 967 22951 1366 11745 5556 6926
2805 18271 10046 4277 207 19518 17387 9701 8515 6813 10532
19714 21923 13493 1768 18819 6093 14086 13695 12781 9782 445
22160 15778 13629 10312 19769 8567 22096 15558 19730 11861 18492
10729 16847 273 4119 4392 11480 20396 3505 7220 390 5546
17277 8531 17390 22364 7167 2217
7325 3832 19899 21104 8400 3906
6218 20330 14943 14477 5614 1582
21534 14286 14624 14809 6775 22838
15786 6527 15848 5288 13523 9692
12696 15315 602
17081 6828 13578
3492 6510 20337
6113 5090 7290
20122 15539 19267
10412 19090 17863
2546 2295 19448
20296 2296 2627
6740 14224 10460
12878 6055 15452
15152 15699 563
15414 21900 19161
11126 15975 3733
4379 15742 6475

ETSI

66

ETSI EN 302 307-2 V1.4.1 (2024-08)

17203 5870 18537
4912 260 21115
23164 4273 1694
1082 5287 11152
14537 2277 19232
13414 15608 12926
17043 18241 18313
208 6118 20777
9140 19241 22845
18527 5035 4161
20867 22650 5585
7875 10358 1898
3563 14833 21329
14705 3359 13959
4507 11976 20017
22424 12925 8308
8739 15561 8010
6408 20723 20928
12337 7864 15777
12742 20430 17351
6259 1865 9808
8343 17441 2551
2167 3025 23181
22718 13243 4797
4223 4982 4395
1609 16748 17625
8463 15204 19632
6583 9112 20284
11334 19370 4763
746 18560 15222
8796 12725 15176
10245 15567 9991
17447 18373 21523
1473 5286 15793
17675 21170 6699
15515 15942 8733
7047 11348 14584
20435 19603 1961
18851 7069 11402
19180 6487 2979
2650 13282 9040
22613 23266 4786
20832 3001 23129
3850 5255 6601
19827 15438 13956
15798 4430 11318
4724 8719 21209
18127 844 21379
7427 22987 10233
22949 8145 21778
7622 14471 18874
8566 14340 3381
3373 419 11514
15127 917 13136
19375 18740 4951
960 2856 17804
662 8107 10298
10993 11755 19142
11400 18818 521
7210 18658 8285
9496 20836 5655
14654 13694 12705

ETSI

67

ETSI EN 302 307-2 V1.4.1 (2024-08)

20381 16473 7271
12796 3280 23370
13893 7667 1736
5485 18321 7789
11242 18771 17282
817 21060 15985
666 20461 22464
7696 19774 4324
12239 14014 4759
5011 10472 4137
3047 2444 3818
1594 20382 538
7051 21874 1697
18539 26 21487

Table B.8: LDPC code identifier: 25/36 (nldpc = 64 800)

11863 9493 4143 12695 8706 170 4967 798 9856 6015 5125
12288 19567 18233 15430 1671 3787 10133 15709 7883 14260 17039
2066 12269 14620 7577 11525 19519 6181 3850 8893 272 12473
8857 12404 1136 19464 15113 12598 12147 4987 13843 12152 13241
1354 12339 4308 23 12677 11533 3187 11609 4740 14630 19630
14508 10946 3928 580 3526 17836 3786 15739 13991 1238 1071
6977 13222 13811 585 8154 2579 8314 12185 15876 7738 5691
12901 12576 11597 4893 17238 15556 8106 12472 10455 14530 17432
8373 12875 16582 14611 14267 15093 2405 9342 18326 12125 9257
5861 12284 2441 13280 2762 5076 17758 4359 6156 18961 13208
4400 8474 19629 19528 14125 12780 12740 19316 491 4761 1719
7270 6615 1175 15848 6943 18360 8905 13921 10807 19688 18757
8312 12234 17907 17254 7699 18399 5508 12215 4818 18107 2874
19496 13973 10432 13445 15320 13648 1501 10549 6710 8897 1998
1575 12713 10916 5316 13713 11318 4055 5782 5828 17981 3141
12177 10726 4244 3138 15996 6822 7495 5257 8909 6180 10680
6650 1909 19146 1038 17229 10050 3051 9793 10839 3532 14759
5337 8448 4939 14792 7585 17860 8612 2229 18965 1519 2031
13845 9320 579 15441 15050 752 8303 6989 13360 12927 15255
17286 3639 1733 16883 8457 9475 2939 3234 1993 8554 9939
6359 15474 12100 6992 13844 16988 7481 16977 9052 9262 15270
7181 3624 3814 16379 182 4338 17627 3315 5745 14093 15574
10709 18662 6909 11248 5268 412 5854 16782 16059 10498 5061
13321 617 6734 3718 15441 19241 17214 1682 18641 18646 6330
7377 16951 14477 6507 9922 11464 2563 5702 12691 10606 17874
7198 12571 17617 4862 18899 7100 8130 9665 10779
6789 11459 17651 3693 13332 3854 7737 12589 15189
16260 14569 9442 17890 18097 6845 6960 1376 8099
12719 14986 18999 14013 3449 13618 14807 265 1508
11231 966 15957 8315 3384 2570 5700 10911 17372
153 8445 19598
7841 14806 54
2492 14099 11718
18608 4278 333
59 3982 16986
3494 12496 2775
18320 10650 16234
9739 16537 19706
7587 19072 18775
14133 12042 2922
229 17958 15889
5130 11029 271
5122 7021 7067
12258 16611 9245

ETSI

68

ETSI EN 302 307-2 V1.4.1 (2024-08)

15493 15347 15939
741 12055 2822
12804 3480 5690
18598 19273 16354
2569 16771 13693
15051 853 956
12256 2756 15137
15685 2802 16479
14687 12470 3583
15473 17781 867
4843 6765 13122
11287 3680 19101
4609 11385 13470
12353 6632 206
10984 3116 1263
9419 14455 19438
9528 1808 435
2238 12870 10119
10868 8402 11111
11081 7197 2667
13780 10759 19722
3768 3052 1836
446 1642 12388
16876 8398 14485
7301 14815 13811
5678 10419 14396
1877 14384 12817
19028 19589 6893
8725 6346 676
13611 12486 2054
11203 14908 14692
18139 5334 1253
16233 9749 16946
18885 4332 16306
3862 10395 13871
3747 8900 3381
13367 14132 7220
15095 4219 15869
13519 18079 17541
19012 13943 19471
2221 5710 13711
5185 3363 10195
9580 17331 15360
14387 7596 9614
17336 6371 6030
14629 10636 10159
2402 9170 4321
1040 5899 153
7710 7637 13966
10919 8535 3791
1968 2567 4986
4166 8744 17691
540 10695 10019
17710 1188 10821
5858 17012 17389
3083 17587 12682
5354 9537 6807
4964 15942 9653
9000 17053 13291
11685 8503 10777
13919 18155 9877
1625 15314 13879

ETSI

69

ETSI EN 302 307-2 V1.4.1 (2024-08)

18520 7074 17061
3748 2752 7298
493 19163 14139
2260 18339 10688
8928 17695 10276
7640 18547 3561
11275 5297 13167
19691 19542 15725
11837 7273 11297
17873 7840 19563
8109 3811 18417
17759 17623 13175
10041 4152 2249
18452 1450 19309
9161 11651 4614
11547 14058 639
9384 3272 12368
5898 2578 14635
15963 6733 11048

Table B.9: LDPC code identifier: 13/18 (nldpc = 64 800)

2510 12817 11890 13009 5343 1775 10496 13302 13348 17880
6766 16330 2412 7944 2483 7602 12482 6942 3070 9231
16410 1766 1240 10046 12091 14475 7003 202 7733 11237
15562 4695 13931 17100 11102 770 3848 4216 7132 10929
16469 17153 8177 8723 12861 15948 2251 1500 11526 8590
14813 3505 12654 1079 11736 6290 2299 17073 6330 5997
390 16492 13989 1320 14600 7061 6583 458 894 1596
8625 7644 1322 16647 15763 10439 8740 5529 2969 13893
13425 13121 5344 8739 4953 7654 17848 9334 9533 2731
12506 10992 8762 5395 6424 11688 3193 17601 14679 8204
5466 15487 1642 6671 13557 4074 7182 4436 12398 12973
1958 13041 6579 15984 3762 16633 6113 11509 7227 28
17202 4813 14024 15099 2648 4476 2260 6507 9930 9232
14186 14510 6818 7665 12708 2645 16687 13255 8239 15884
1751 7847 17987 11410 3345 17133 17655 5027 1261 17191
8056 4264 13915 8217 6118 8072 6278 6835 5038 15008
13625 2999 5336 11687 13500 5723 13903 766 6293 155
12316 14093 7372 16846 15357 9865 17869 1429 16681 202
15062 1123 6454 17625 3213 39 1669 1770 13636 16555
13053 7597 11481 1336 3343 11387 5463 17830 13741 5976
1956 13509 1664 16867 8168 13421 17078 3285 17138 1572
16711 1499 4805 13584 14759 2844 13110 7356 5850 8330
6521 8528 14170 6681 16992 12867 14326 15227 4082 8595
16176 8184 8572 1923 935 8900 13020 6812 9778 3391
3946 4711 15314 15108 15634 4144 4372 9207 10715 1291
16601 5864 10968 4724 9235 6988 3307 6515 7004 16328
16217 4227 9735 15857 5003 2532 4451 8574 2149 6908
9506 8949 12035 9701 3124 14295 8567 13614 5159 16746
2418 8669 10921 5738 147 1004 2692 9065 12877 7559
16706 8511 10314 3118 1219 7071 12376 538 2389 3297
12492 10589 5791
13528 1653 6618
10485 1307 4102
347 13580 4039
523 10311 10540
4183 6192 17159
11458 6521 9632
11594 15791 10384
11654 126 11715

ETSI

70

ETSI EN 302 307-2 V1.4.1 (2024-08)

6265 34 5091
7271 13900 7588
3960 11297 1612
9857 4695 16399
6423 2197 15040
4219 5979 13959
2959 578 8404
4585 658 6474
15900 11357 5249
7414 8642 1151
4130 9064 14537
14517 1356 3748
13865 12085 17295
9530 5110 1570
10862 8458 15322
16355 1774 5270
1229 11587 1632
17039 787 4703
11423 15388 6136
8413 9703 13946
4678 4072 16702
6244 4690 7164
7238 14169 5398
8679 122 11593
10954 15802 16427
9413 6717 16406
1027 17863 7836
655 8827 10286
4124 12599 12482
12955 3121 15318
8343 16634 6301
13568 5056 9920
1948 10 17395
8550 131 2151
15226 15994 13093
10966 15412 2781
13425 15831 5346
2261 1067 6346
6625 1966 13533
10575 4483 5761
14366 2019 14426
16746 1450 4830
13109 7358 7942
15376 7284 14035
14341 12625 3306
9375 7529 1537
13831 13447 4549
15658 15299 8238
4005 13264 9766
4715 6285 15383
1262 12883 15434
11123 14975 3434
5307 1112 16967
12163 12009 3681
9174 13153 10344
13456 13197 9562
1785 7549 15347
663 9748 9436
4961 11903 11574
16248 6238 666
11426 13748 14763
14431 1443 2069

ETSI

71

ETSI EN 302 307-2 V1.4.1 (2024-08)

2376 8154 14978
13140 1289 9046
1159 300 3319
11510 7769 15877
6430 14946 6856
8868 15622 12458
4867 6622 6850
14721 11241 12760
14233 9874 17682
16677 13195 15086
11155 7067 14160
12741 14379 8922
1930 17055 11752
12361 6523 9568
12165 5636 16011
11389 4754 9916
15903 15542 8301
12073 4918 9754
16544 17907 14814
10839 1401 5107
12320 1095 8592
15088 6521 12015
14802 3901 8920
17932 2990 1643
5102 3870 2045
540 2643 2287
5844 2482 9471
10428 637 3629
8814 7277 2678

Table B.10: LDPC code identifier: 7/9 (nldpc = 64 800)

13057 12620 2789 3553 6763 8329 3333 7822 10490 13943 4101 2556
658 11386 2242 7249 5935 2148 5291 11992 3222 2957 6454 3343
93 1205 12706 11406 9017 7834 5358 13700 14295 4152 6287 4249
6958 2768 8087 1759 11889 4474 3925 4004 14392 8923 6962 4822
6719 5436 1905 10228 5059 4892 12448 26 12891 10607 12210 10424
8368 10667 9045 7694 13097 3555 4831 411 8539 6527 12753 11530
4960 6647 13969 3556 9997 7898 2134 9931 3749 4305 11242 10410
9125 9075 9916 12370 8720 6056 8128 5425 979 3421 5660 9473
4348 11979 5985 395 11255 13878 7797 4962 13519 13323 7596 5520
2852 8519 3022 9432 3564 9467 8569 12235 11837 5031 4246 2
4081 3630 1619 2525 3773 11491 14076 9834 3618 2008 4694 6948
7684 9642 5970 1679 13207 12368 262 7401 11471 2861 5620 4754
7474 10418 1422 10960 13852 988 13465 6415 86 2432 7595 12239
8539 11749 8794 6350 1947 13325 13061 7385 13017 2536 13121 15
7944 13831 5126 9938 11758 335 980 9736 12143 5753 4533 10814
10706 12618 6949 2684 4107 14388 11372 6321 13832 9190 2838 13860
10830 1947 13803 3257 2677 406 8400 10536 12911 3629 251 9784
13343 13304 301 801 6456 6351 6155 6763 3812 11337 8446 9306
524 5573 503 10544 8990 673 2309 12376 466 11441 960 1557
4403 3564 1732 13453 12054 8941 1383 12424 4347 9830 3553 5158
2025 4282 4983 13553 10776
11833 13099 5078 4420 3527
1544 7474 2780 7749 4153
11189 520 8463 12230 7712
10409 13367 2604 2966 9248
1412 420 3507 9818 7955
1122 12483 9375 10232 9456
2799 7033 10404 4495 12059
2569 5970 6262 2199 8045

ETSI

72

ETSI EN 302 307-2 V1.4.1 (2024-08)

11724 511 12693 12855 9597
756 12900 13391 13623 10683
2095 13479 1488 9469 11142
13849 1356 10776 3530 9866
13449 14225 2072 12772 9461
6466 6181 6502 401 7439
4631 1086 3062 11789 11811
6788 14007 2270 14132 2764
4643 10272 11316 2608 8511
5221 9028 2736 7223 1051
1974 2737 6739 13904 6156
5 9082 3915
2400 7195 3413
606 221 8171
4548 1267 5310
12795 2160 8305
10563 3507 12190
6325 2499 9717
9251 6046 13308
11704 10834 11241
4777 3774 11533
12487 10365 6852
58 2650 2027
7248 13704 5573
12777 7834 8561
7906 8121 7774
554 3105 6000
11198 3586 10410
9002 4094 11297
12058 1037 13638
1258 12917 11078
2430 51 10276
7841 9451 10236
11045 1058 10352
9629 9428 86
8146 1255 3802
10820 6337 4199
9364 7723 1139
438 6445 583
2683 5358 10730
8471 3061 13380
3005 2840 4754
8210 1814 11502
8667 14258 5985
8407 13336 10970
6363 11715 5053
104 13618 13817
6562 4087 294
1742 10528 4626
6607 2692 1587
11097 8361 2788
13451 3541 823
4060 13604 9816
157 6106 1062
8853 5159 4270
9352 13164 2919
7526 5174 12501
12634 13077 5129
5750 1568 6281
269 5985 10973
8518 9415 1028
4722 13275 634

ETSI

73

ETSI EN 302 307-2 V1.4.1 (2024-08)

12113 7104 7436
12787 1032 5936
3425 11526 10797
784 9208 15
11223 12849 4913
10635 3553 8852
11749 10619 3532
4080 9831 9219
6560 6049 6111
1304 11770 12585
13209 8589 11287
2887 10699 14307
4752 456 4073
1175 13156 4894
12756 3237 6279
10125 7074 2344
7533 7103 5226
4000 4425 12173
10056 5312 1599
7445 8696 12533
11509 14050 2483
12405 2876 5033
4512 4955 5627
5572 5099 10987
10665 404 3082
2075 1583 13454
5666 7228 524
13290 7634 418
9006 7368 4181
9447 3674 8171
9355 10211 9342
12572 3681 3322
3295 186 7491
7926 212 5241
5479 1654 8097
5078 423 4817
1357 12780 3664
11900 402 13108
299 7166 12008
5750 3041 5618
8357 1229 8884
3713 8791 13375
4390 6302 568
1009 4440 10003
1209 11978 11711
1803 9838 13537
11318 9750 12421
2388 3021 7880
7220 1062 6871

ETSI

74

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table B.11: LDPC code identifier: 90/180 (nldpc = 64 800)

708 1132 2120 3208 3879 8320 11948 14185 15214 16594 17849 19766 23814 26175 27579 28052 31512 32029
2720 2753 3716 6133 8020 8305 9429 10337 15503 19905 20127 21963 25624 27221 27907 27945 29833 30270
4011 7807 11547 12782 13040 14599 14836 15218 17890 18922 19668 20267 20714 22151 24373 25261 26101
27627
136 5341 7661 12779 13392 13922 14151 15054 16544 17232 17478 19895 22814 23820 25014 26346 27575 31803
3456 3485 5839 8312 8423 9796 10018 11520 13336 15520 19928 22019 23144 25339 27406 28729 29527 31406
1779 3634 3930 4138 5449 5924 6776 7658 8703 11542 13133 15086 16334 21876 23860 24281 28854 29557
697 868 1345 6257 7400 8220 9761 11501 15828 16175 16865 17251 19298 21907 24033 24175 24497 30965
991 1845 3173 5609 11275 12666 12903 14409 15359 17537 17923 19821 20323 21561 21663 23378 25371 28487
446 3096 3604 3688 6864 7296 8128 9957 11568 13204 14502 16280 17655 19695 25953 28006 31006 31160
3592 5443 5450 8875 10529 10721 15241 16485 16905 17980 19685 21639 21938 25038 25322 26073 27072 32305
2539 11274 18981
8099 17427 18613
7872 12547 14776
17272 31146 31476
12171 20651 28060
5845 20532 24021
2102 9627 12746
4406 13397 16767
7707 19199 20221
10278 11526 13739
8902 13337 25524
5385 6939 15968
1686 2985 18124
21285 22673 25722
4833 4895 7657
14073 19518 27540
2832 27137 32072
8936 19641 24786
1696 4408 9480
3941 9228 25043
1328 7460 25237
11251 21361 23890
10450 10762 26795
1544 19244 22553
9564 24279 27073
12214 15608 30892
6316 29252 30504
3667 7784 26600
11435 20278 31840
7211 21620 23899
17193 18368 23536
3750 18865 29121
2088 7648 20893
12231 28534 28933
6316 14450 31885
2672 8770 26920
17337 18624 26359
3098 17939 27134
1084 24695 31846
5110 9148 10203
3943 19212 31745
6 6061 29453
2271 9151 27015
386 2747 26182
13129 15456 30698
126 10222 23935
11008 17244 19711
17752 22103 31308
11293 20670 23940

ETSI

75

ETSI EN 302 307-2 V1.4.1 (2024-08)

11627 14829 19929
2163 5918 23666
28627 28709 31369
3161 3209 26227
1597 25322 30792
2083 15971 16193
4795 10875 11668
12984 28077 28863
1851 9750 30222
2252 8660 8984
6764 8375 15896
5074 16399 31725
11507 15519 30828
3196 7975 17614
477 11889 17748
2420 2852 25451
3683 4741 6944 8199 8808 13142 14071 15830 17216 18589 20284 21652 22542 24994 25719 26187
1534 4620 4899 6461 6737 9082 10679 11544 16118 20173 20662 21526 22646 24778 29790 30044
2343 2547 5620 6523 8188 9029 14162 15517 24143 25078 25163 26616 28731 30201 30889 32034
1876 4541 5147 9087 12853 12967 13887 16009 19722 20475 21245 21908 22547 25790 27330 27640
1706 3168 6471 7382 10195 11568 11770 17719 19484 19572 20375 20470 23332 24372 30654 31230
996 3297 3587 4313 12243 12456 17510 20492 29071
7114 7312 7937 8379 8580 11514 13657 23774 24569
98 600 745 1223 4298 6362 12544 21620 28786
2585 4021 10785 11294 20707 25033 25465 26990 30713
1895 4346 10404 16998 17073 24131 24891 26056 26444
4265 8641 8937 13462 23815 26205
1468 2578 3070 6258 8221 10414
5186 8832 11589 25697 29629 32022
15971 17493 18659 19082 22089 26779
1597 1691 10499 13815 18943 27396

Table B.12: LDPC code identifier: 96/180 (nldpc = 64 800)

551 1039 1564 1910 3126 4986 5636 5661 7079 9384 9971 10460 11259 14150 14389 14568 14681 21772 27818
28671
384 1734 1993 3890 4594 6655 7483 8508 8573 8720 10388 15541 17306 18411 18606 19048 19273 21492 21970
29495
1104 2877 10668 11101 12647 13994 14598 15653 17265 18435 18848 18949 19209 19312 19414 19465 24927
26613 28809 28865
1185 6439 6519 7790 8609 8826 9934 16363 16596 18088 18757 20318 20446 21123 23938 24235 25120 25469
26036 28043
53 3630 4384 4619 7805 8822 12208 13312 14269 16435 17925 18079 18689 19042 21541 22729 26066 27666 28759
30107
1926 2549 9816 10544 10980 12468 13026 15658 15670 15975 17200 22364 22775 23343 24289 24956 26230 28040
28348 29718
1243 1673 4181 6080 7829 8259 9294 10556 10905 14071 18080 18203 18537 19707 24143 24442 25877 27072
29158 29690
1834 2523 5973 6006 8054 8843 10141 11668 12956 13202 18659 21757 24258 24675 24779 25924 26980 27008
29229 29899
3790 5716 7345 7381 9081 9679 13629 18038 19858 21248 21348 22251 24410 26790 27643 27955 27996 28271
29638 30198
158 545 1178 5181 8585 9927 10721 11361 11640 12552 12579 14641 14928 15609 17388 20551 24516 26834 29850
30201
1076 3011 5636 6947 7236 7511 10629 16795 20770 22796 22853 24219 28516 29151
678 2553 3403 6515 7079 8767 10228 10791 10832 16113 18718 21328 25762 26925
8536 8706 9471 9854 11186 12220 13261 14034 14897 25068 26338 26427 28784 29281
2634 3562 11652 13206 15185 17051 19666 21058 23107 23869 24590 25311 25498 28891
2440 4164 7040 7591 9321 9796 11026 12204 13478 17852 22183 25257 28756 28813
390 2209 3095 4554 5755 12285 12314 14372 14957 15711 22946 27713
207 418 3856 8719 11708 15353 20717 21639 23269 26732 27613 28334

ETSI

76

ETSI EN 302 307-2 V1.4.1 (2024-08)

2805 3795 7105 8130 10330 13888 15132 16415 17093 23277 25191 27630
1381 1955 3689 6290 6342 9573 13751 14633 16298 18206 24248 24893
5991 8976 9763 12308 12566 15265 17068 21084 22475 24371 25171 28008
8962 17060 22260
1335 6691 14738
4118 10315 23113
4643 10970 28091
1195 3683 26231
7486 17403 22471
7328 10110 19764
4630 13732 28298
6139 19386 26009
19712 20670 27993
9317 18037 19324
4422 4510 10290
1396 22324 28601
1404 5708 22352
14783 17214 19359
7996 20133 20614
6219 17582 24404
2481 20505 28124
4569 10863 28776
941 7516 11474
24878 27151 28125
9053 10186 28126
1376 19327 26055
5965 14239 16875
17434 18098 25044
5785 14385 22243
15144 16981 25171
13671 25732 25961
95 20461 20558
4321 19572 26175
3637 17351 18683
18096 23926 27359
7914 13217 23098
9822 11342 26728
7591 18615 28044
112 16897 19127
10087 18516 27292
2762 3323 21677
4533 20728 23071
7164 7180 15097
2061 6184 20598
6310 13462 26714
8189 9345 15315
3319 17370 24726
5217 9271 11984
10245 13623 16621
5537 22325 22692
1592 19859 25578
7005 15625 22572
1474 14387 28592
153 4254 20080
8709 25107 25135
11663 17264 25598
7135 17226 18698
109 2603 26360
1408 9608 11927 14872
4187 10410 27776 30125
1008 4409 14938 20458
3641 12480 20141 22605

ETSI

77

ETSI EN 302 307-2 V1.4.1 (2024-08)

10453 22378 24993 30002
19957 24800 25196 29823
2431 2929 5824 12333
395 4393 15571 22019
15040 24712 28275 28794
3735 11988 13828 13949
4301 5829 24675 26696
6406 8140 9438 17274
4272 17638 19278 24961
3271 11030 11481 28624
3792 5394 7566 17557
6505 11108 20811
2722 28613 28991
11472 25911 30170
2355 13553 25490
3284 13373 17330
9046 16513 22243

Table B.13: LDPC code identifier: 100/180 (nldpc = 64 800)

690 1366 2591 2859 4224 5842 7310 8181 12432 15667 15717 16935 17583 19696 20573 21269
2488 2890 6630 6892 11563 12518 15560 16798 18355 18746 19165 19295 21567 23505 23617 23629
321 2844 2894 3986 4538 7599 7816 9831 10247 11556 16068 17249 18194 23043 23100 25938
2503 2827 4771 5929 6400 7437 8054 10897 11633 14404 16133 17101 24425 24973 25086 25802
1462 2099 3910 5131 5352 8832 9495 9624 10796 12906 13903 14724 14946 17708 21034 26612
260 523 1427 3435 4517 9494 12594 12688 12726 14163 16537 17424 18424 20321 25101 28269
2131 2407 4820 7167 11783 15249 15982 18761 22162 24593 24971 25831 26351 27005 28348 28793
2089 5829 6119 7505 7758 8122 9870 12107 16656 17582 19115 23738 27646 27692 27862 28356
2714 3288 3337 5013 6210 8080 9348 12919 13458 13621 18015 21024 24044 24761 25610 26317
1305 3402 5830 7095 8852 9580 9793 11157 12725 14355 20659 21400 22289 23823 26250 27006
12936 15702 23593
3616 17219 18621
1234 12759 26749
396 3017 18360
10664 21597 26165
12986 14553 24818
18403 21213 28302
6515 18331 19413
19649 26219 27483
2538 15793 17528
7871 9374 20785
5494 8084 21558
6691 7770 14306
3247 7417 18827
11615 15987 20081
1527 15272 26042
10540 15548 23849
223 2601 25888
2395 21465 28501
19703 21589 27252
12832 15726 25300
3750 10030 16124
401 6474 28682
4424 19876 25563
590 12853 14779
25185 25539 25920
6857 23329 25764
3467 23205 23751
9278 24364 25033
14956 19104 22487
21856 26163 27130

ETSI

78

ETSI EN 302 307-2 V1.4.1 (2024-08)

2067 17357 22055
50 14414 19142
306 445 16437
2260 13892 17608
8893 12230 16916
5943 8921 16380
5079 15385 21951
5108 6038 8513
2126 6749 7330
3814 11941 22949
2301 15416 26731
3498 14463 20417
2062 10434 10746
18128 18960 23452
13080 13129 27193
18818 24995 27087
7198 11948 23135
17206 18524 25811
5202 10030 10076
8497 23410 23991
1553 1968 13135
4426 10786 23259
92 7941 23045
6356 14028 23104
18319 20286 22984
5778 25335 26191
662 15922 27478
2920 9733 18974
15337 27509 27519
8659 25028 27723
14865 24564 26361
1383 21234 21770
10767 25752 25843
7717 14536 24248
278 2803 2966 3547 4128 4829 4981 6699 6716 14183 14239 15939 16996 19694 20073
3022 3087 10039 10174 11403 12146 13689 14934 17765 18121 18936 21818 27202 27532 28192
817 3888 4102 9441 10165 10941 18131 20028 22305 23832 25225 26228 27208 27245 27390
6346 7992 9053 11187 12124 16435 16850 21269 21580 22096 23700 24751 26264 27318 27576
1440 3291 5755 12247 12272 15394 15659 15764 16338 17373 18840 19597 19812 22415 27062
937 3118 8745 10933 12703 13906 14113 21442 21539 28140
247 2465 2918 3189 5886 11451 16862 17458 20908 26608
58 10104 11815 14429 16531 19797 24071 26021 28000 28489
4367 5710 7855 14203 18071 19336 19880 20166 26774 28554
191 1085 4068 7452 11739 15962 17501 19172 24130 28476
4961 19716 19964 23479 24004 24340 25537 27930
1688 2235 10464 15112 15134 25143 25910 28689
765 11839 17427 19754 21445 22034 23493 25296
277 7947 9952 12228 12595 16563 19758 21721
1575 2652 5226 8159 16624 25446 26417 26722
10571 17389 22602
1331 7875 18475
11738 13853 23914
9412 11361 26507
16877 23022 27060
2627 16649 22369
9446 14752 28540
4496 7705 22247
2439 19741 28550
6605 12623 26774

ETSI

79

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table B.14: LDPC code identifier: 104/180 (nldpc = 64 800)

2087 6318 7314 8327 9453 12989 13156 13763 13819 16963 18495 19352 20510 20651 23379 23847 23953 26469
2680 5652 6816 7854 10673 11431 12379 14570 17081 19341 20749 21056 22990 23012 24902 25547 26718 27284
2142 3940 4724 4791 6617 6800 9349 9380 10073 10147 11750 12900 16044 16156 17769 21600 21669 22554
1588 3097 4277 6181 6737 8974 9793 12215 12814 17953 18270 21808 22625 24390 25429 25750 25967 26391
561 5825 7106 7166 7475 11844 12905 13559 13978 14176 14437 16070 16587 19792 20187 23754 26070 27232
673 1783 4046 4887 5596 8390 9229 12315 14252 14415 14529 17837 20013 20032 22201 22487 24412 25792
1261 1910 3767 6244 7050 7367 9230 12972 13229 13472 14287 14494 16776 20523 20738 21591 23622 25206
1618 2106 3640 6304 7984 8158 9072 9311 12618 15746 16985 18923 20959 21267 23375 24052 24260 24827
6256 6931 7276 7356 7832 12284 12405 13083 13602 14750 19021 20026 22661 23283 24427 25301 25982 27279
2432 3076 3399 5305 7370 8406 8826 9237 10537 15492 15606 15619 16515 17562 19550 22525 24389 25740
157 296 422 467 7125 9849 9997 15376 15506 16119 17153 17857 18639 23136
1275 1439 6162 8258 9031 10207 10472 16004 16641 17140 21342 22191 23200 25753
110 1073 6460 9208 10520 15833 15951 17494 18614 19970 20537 21512 21796 22135
3771 5399 5885 7905 8302 8614 10205 11133 11459 16044 22701 25170 26255 27086
1597 2640 2741 3790 5107 7470 9160 12078 12350 14020 18877 19507 22658 24290
4957 5961 6263 8201 8579 9392 10133 11712 14757 15678 15718 19528 25107 25122
870 4508 5944 7360 11724 15003 16387 19543 19893 20189 21942 23740 25686 25849
131 2044 6731 7619 7787 9109 9841 10006 10275 13687 16522 18212 24457 25197
504 1863 4246 5075 5448 6296 6930 11792 13736 14588 16340 17102 17807 26621
1137 1168 2366 3818 4311 6806 8583 10850 12198 12357 21357 23243 23568 25003
2353 11886 22548
1680 9112 12175
15126 16642 27030
5571 5719 19190
6232 13413 19728
8197 12068 17122
3220 3476 24534
1630 4345 23890
19815 20676 24443
12761 14080 15937
41 7963 23895
7658 13020 27128
1017 1447 3285
2268 22921 26255
261 13889 14175
13925 18777 18987
15136 24523 27156
12008 18484 19299
4304 9857 15134
2966 9278 9737
5469 15449 22259
11359 14186 20635
16453 21262 23629
5613 7100 11104
3959 14714 18116
7465 13803 24660
3939 7615 9891
12249 16491 22373
8734 14253 25616
5781 18719 23894
6208 6703 14626
1284 4730 23920
3920 13167 13366
3925 7147 27268
1926 12777 21092
675 8186 22557
487 9590 12433
7090 16031 27037
3083 10445 22950
380 4663 7195

ETSI

80

ETSI EN 302 307-2 V1.4.1 (2024-08)

960 12754 20597
1790 12476 24250
11307 22121 22266
3256 7625 12046
11034 11800 17383
6142 14781 19944
2679 11106 22783
7769 11060 15178
7384 9851 20205
14813 19271 22600
3085 11637 19934
6518 7995 19382
11070 15498 26380
248 16291 23824
4989 19436 26642
5954 16039 16042 20349 21326 24656 25427
2558 6628 9167 16825 19069 20808 22617
317 13859 14069 16104 18835 20040 26633
2866 4153 5875 11698 15287 19719 25808
536 6955 9735 16098 20694 24675 26881
25 7316 9961 21037
7823 19458 20404 25186
7142 11057 17748 24788
11315 12358 21583 21836
8995 9326 12826 25981
2281 10560 10674 19801
5001 6655 26231 26542
800 15131 18482 22621
9060 12257 24786 25188
3462 17201 18960 24462
17631 26360 26425
12774 20967 21391
14701 20696 26807
5931 13144 14022
128 16460 26300
801 9487 25937
6153 11296 23054
2749 14434 20049
1732 7646 20402
3839 11031 26022
2159 20918 21407
285 13785 24234
1977 3899 7972
4120 19101 23719

Table B.15: LDPC code identifier: 116/180 (nldpc = 64 800)

3880 4377 6147 6219 7873 8180 9157 10311 10862 15393 16522 17318 17609 18398 19290 19293 20296 22244
1056 1647 5119 5201 6991 10038 10843 11614 11901 12026 14631 16749 16772 16915 17331 19235 19877 22763
501 2634 2812 3085 3242 4952 5087 8334 8838 8993 12601 12849 13142 13852 14416 14444 15122 20692
343 1183 5708 6798 6951 9154 9160 9508 9884 11874 11984 13737 14933 17208 21253 21822 22723 22898
3332 4384 5137 8527 8749 10414 10536 12759 14769 16121 19255 19326 20283 20352 20629 20827 21226 22087
60 3866 3895 4116 5631 6985 7205 7681 10031 12825 14266 14644 16396 17010 20221 20268 21729 21752
61 1112 1392 1826 1910 4370 5910 6660 6943 7859 9628 10213 10701 12615 14453 17123 18667 20688
880 2397 2669 7767 9683 9705 10430 13995 15972 16217 17187 18246 18869 21077 21884 21897 21927 22475
748 1029 1446 2912 6784 6926 7710 11674 12014 12409 12438 14411 14723 15953 16020 17496 18230 19547
1151 2295 2975 3082 6502 8269 9130 9629 10018 10235 14871 15834 17396 17777 19138 21871 22035 22927
650 789 4632 4777 5004 8796 13479 14917 16826 16926 19144 20754
1693 4906 5777 5907 6472 9792 11824 16134 16406 16440 18395 22338
5172 5920 7987 9381 10573 11382 11512 13074 15755 16591 19518 20968
1409 2508 6542 8993 10660 13691 14724 15597 19641 20809 21160 22767

ETSI

81

ETSI EN 302 307-2 V1.4.1 (2024-08)

895 1446 3298 4018 5250 6269 8897 9049 12052 15311 16199 20677
1 774 1248 2362 7019 8430 14321 14444 19664 21475
1714 1973 4155 7536 7975 9323 9997 10627 20959 21824
586 1907 2153 5914 7407 8311 8900 10060 18502 18818
805 1897 3019 7404 10055 11779 11982 15319 21802 21913
5276 5470 8725 11080 11939 17057 17960 18930 19814 22546
1227 10140 18999
849 17266 18364
4436 6167 14942
11103 14219 19204
6738 10043 20614
1885 3173 13934
2088 11344 20627
2668 6722 20336
11274 18439 21280
2223 15960 21282
6555 7521 11051
9037 11912 22911
12952 19885 21298
13696 16793 17228
1040 4501 6170
1025 4522 21287
1213 3817 12857
1392 6601 12468
835 16504 19633
634 16014 19619
6166 17343 21067
6583 16107 18382
5481 9653 18543
14634 15406 16179
1952 7810 16892
2271 12635 20456
8838 10469 20629
11400 16788 18756
230 11373 17104
17204 17733 20707
8465 13092 22087
8684 8983 10130
11468 13469 21366
9342 10115 19130
3184 9535 11802
13495 16231 19609
8911 12617 15190
508 8674 19422
4059 6197 8449
9440 11028 13468
1779 9358 13459
46 7370 15160
12118 17458 21853
320 4449 20048
12300 14502 21803
9019 19417 22280
1320 6434 7916
6850 10275 17099
301 5637 7309
8443 13673 16625
4943 15268 20252
13246 17809 18271
3230 8329 12330
1398 7959 18145
274 10500 12954
1326 2057 5453 6588 11514 11920 13687 14692 17684 22043

ETSI

82

ETSI EN 302 307-2 V1.4.1 (2024-08)

3921 7217 8693 10943 11769 12121 12618 19781 19932 20083
2166 5206 5482 11453 13986 16877 18184 18822 21663 22611
858 11727 13116 14705 15517 16109 17086 18439 19047 20321
216 414 726 2616 6948 7028 8288 12164 21697 22606
7441 14498 15308 17321
1455 6627 10112 13652
7448 7945 17043 21758
2947 7933 11624 14237
514 4014 20406 22226
4454 9815 11696 13946
7787 11797 13113 15796
2363 4379 21733 22277
8437 16504 16527 21350
8932 14444 15680 19635
1273 11365 15443
3533 11221 13249
687 1927 14403
3914 4221 8791
12479 15622 17384
14928 20923 22283
7729 13750 15716
88 12409 19522
6852 16166 21884
1204 12049 16487
11178 11226 15971
6382 14361 16863
10195 10247 18188
1819 5834 8434
286 3333 21431
13950 15188 17771
10198 14887 16751
13811 18307 18337
1210 18076 21869
5717 8482 11896
6501 15625 17792
3965 4494 20272
1589 9900 14472
288 9421 12009
2177 4626 16605
710 4696 18127

Table B.16: LDPC code identifier: 124/180 (nldpc = 64 800)

1083 2862 3815 4075 5519 8003 9308 10029 12476 12949 13759 13918 14303 15028 19737 19953
392 3781 6086 8378 9952 10531 11369 11954 14808 14948 16585 16682 18445 18960 19085 19423
3023 3727 4797 5104 5342 5994 8138 9758 10146 11758 14763 15300 15640 17947 18690 18864
854 1259 2147 3861 4258 4949 5555 5940 9454 14940 15521 16393 17029 18789 18810 19840
4404 6608 7232 7534 7721 8079 8558 9851 11560 11968 12678 13410 15908 16918 18108 18437
519 1591 1600 1964 7706 9481 10789 11068 13056 13373 13759 14323 14999 15505 17366 18254
545 673 2257 4060 4289 4897 5287 7318 8511 13835 14540 14948 15475 16718 17907 18067
1949 3426 3440 4679 5103 8692 8987 10075 10923 11162 11625 12805 13749 17487 17807 18802
858 1969 2178 2421 2592 2764 3504 7084 9227 9349 9960 10592 18149 18974 19010 19891
3282 5061 5908 6929 7551 7927 8116 8388 11305 11379 12527 13982 14343 15064 16259 19602
3730 8198 8789
1515 6545 9678
12411 14800 17119
1000 15382 18607
977 1525 5376
4464 7676 8937
3684 6730 9836
10203 10305 18629

ETSI

83

ETSI EN 302 307-2 V1.4.1 (2024-08)

2214 4904 10873
690 7077 12623
3094 11228 16285
2583 5278 16500
4253 13495 14465
3323 17768 19548
7670 12892 18704
373 14868 16337
8886 17314 17578
10636 12385 19530
5734 14030 18619
3298 4905 10156
332 19282 19924
15 8741 16429
11482 14807 15426
6055 12144 14026
1095 5737 10525
813 965 4520
808 8546 14057
3195 3814 14683
1184 17287 19477
12411 13207 18549
2639 12198 15656
3527 5555 14387
5563 10402 19122
4538 13134 18766
731 3368 5865
1253 2238 8820
2764 11942 16705
6375 18789 19594
3387 11299 14192
2486 2729 8580
3096 5778 10416
2513 10609 14018
2508 10361 15415
5368 6612 17415
1998 5687 17627
2711 16277 17350
5213 5820 9217
5744 17794 19180
9095 15302 19441
10031 12094 18856
739 6709 11785
1496 10418 15753
9437 11589 19552
7484 9656 12381
2371 7237 7794
748 7213 9835
1930 6418 8683
5482 15289 18623
10791 15731 18781
3622 5730 14230
1169 9420 19665
10170 13288 14142
3855 7239 18843
816 16956 19454
3179 5689 16584
4313 6450 8761 11594 13187 14029 14509 14944 16947 17850 18270 18390 19934
1680 2214 3859 3994 4276 6984 12261 13869 14696 16303 16467 16756 19754
433 1009 3169 6825 7128 7294 7327 8175 16653 16657 17314 18598 19472
1473 2110 2777 5217 5902 7136 7797 8650 9116 11267 14270 15342 18291
349 2892 4148 7493 10142 13920 14079 14423 15229 16255 16808 18248 18362

ETSI

84

ETSI EN 302 307-2 V1.4.1 (2024-08)

5879 7078 7457 9493 10771 11010 12068 12994 13007 13109 17983 19531 20087
483 804 993 1078 1822 4646 4658 5458 8116 8443 10056 13440 18939
490 865 1309 4339 6434 8210 9248 10588 13094 16476 17620 19378 19708
163 899 2396 4212 6157 9036 11116 13295 13928 15111 16312 18369 19470
985 1298 3213 5145 6917 7136 7183 10870 11329 12295 13466 14989 17909
89 582 812 1761 5157 6039 7843 8418 8747 11575 13169 14160
1871 2701 3252 7399 7646 9785 11274 17041 17361 18899 19430 19691
1328 2165 2722 4120 4132 9855 10802 14441 16771 17679 18611 18718
1166 3128 8585 9843 10411 12365 14141 15156 16987 17484 17702 19204
943 952 4108 4832 6706 9245 14304 16528 17055 17698 18419 19526
1340 7429 17768
10358 12400 16483
1070 4760 10051
6992 8645 9886
756 7962 17532
13063 17794 18323
630 9881 20052
5786 7779 15441
5049 5860 16575
10021 13811 20097
2167 6374 19993
1412 4441 11765
14750 17242 18319
507 1756 18791
2277 6901 9690
14828 15959 16658
4687 6452 16770
465 11415 13696
13370 15379 16190
2988 12683 16796
6382 14227 14295
17221 18167 18379
9656 9841 10968
16917 19014 19869
15255 15400 17505
6403 15345 16248
6794 15772 18005
3252 12230 12246
9062 9082 10245
405 9373 19195
5987 6006 6026
2865 2887 2896
14889 14898 14924
7791 7800 7809

Table B.17: LDPC code identifier: 128/180 (nldpc = 64 800)

790 1010 1064 2157 2569 3499 4637 4951 6789 8177 9888 10800 13254 13829 17946
597 693 862 900 4750 4897 5410 5441 6491 8815 11894 13411 13696 14103 18413
903 2779 2996 6100 7489 7560 8637 8853 10078 11372 12040 15911 16944 17059 17771
1761 2084 2099 2232 3114 3120 7062 10527 10823 11945 13918 16359 17110 17654 18370
677 1080 2329 5100 5106 6261 6383 10953 11968 12186 13266 14451 16092 17760 17871
1069 3672 5304 6102 6907 8087 9477 9654 11581 14650 14808 14920 15397 16179 18595
327 1161 2624 4494 4516 5555 6928 7455 7959 8734 8933 9753 10614 16263 17689
1922 1932 6481 7488 7722 8836 10326 10633 11184 12555 13485 14155 16373 17486 18331
1056 1624 1991 3585 6052 7838 10123 11470 14228 15146 16166 16390 17630 17679 17774
295 3429 3587 4597 5017 5105 5136 5827 7308 8266 9632 11612 14879 16167 18491
1523 1615 3368 6253 8510 9070 10020 10368 10718 11882 12014 15077
421 3234 4290 4808 4983 9992 12569 13331 14457 15853 15943 18318
583 2081 4320 6268 6284 9084 9638 10941 13335 15062 17310 17667
573 5180 5758 5813 9655 9892 10763 11209 11717 14760 14972 16395

ETSI

85

ETSI EN 302 307-2 V1.4.1 (2024-08)

151 1917 4190 5573 5629 6725 9653 9974 10008 11971 15132 18170
132 1270 3074 7215 7878 8266 11875 12274 13134 15084 17120 17556
845 2417 2435 5875 7758 7807 12521 13907 16400 17778 18260 18370
2848 4714 5924 6507 7595 8422 9281 13140 13276 14589 15269 15943
278 931 1186 3588 4072 6943 8429 9348 9863 10056 14376 15846
3480 3887 4932 5888 10246 10281 11065 11434 12290 12345 12635 13917
528 14523 18426
4127 5613 9647
8777 15790 18168
3491 5608 10216
5154 8811 16363
437 2834 3470
9675 12773 17150
2456 7748 8623
3758 14333 18097
3969 17136 18610
6745 13708 18656
6152 10273 13316
7822 14888 15541
15501 16598 18531
2497 8828 15453
3443 6899 7293
3721 13954 15822
719 13335 15342
1566 7588 8362
8644 13389 17476
1513 8257 15942
2620 7224 15557
7696 12178 17371
5285 8439 11367
4961 7657 17125
11382 11542 16823
2429 7538 10992
680 7651 10178
6794 11231 18328
1195 12837 15710
1156 17454 18260
6541 10062 17436
70 282 7519
608 1919 7299
3339 11187 15788
4771 12599 13753
1822 4233 10561
5233 14135 15888
4109 14837 18717
3011 15644 17342
10668 11462 15065
2486 6822 7486
3851 6182 11215
595 11064 15525
9738 10045 14128
929 2222 11949
10950 12273 15503
3672 6760 9589
3583 5887 8907
13351 15134 17291
7770 9928 12542
268 10496 17937
1318 2938 6971
428 1791 9729
6895 8896 10420
2946 4619 6209 7377 7931 8740 9223 12171 12985 13795 14141 16233

ETSI

86

ETSI EN 302 307-2 V1.4.1 (2024-08)

217 958 995 3144 5905 6178 6596 10427 15013 15669 16343 18465
357 2579 4550 5223 5890 7642 7900 8441 13416 17740 18131 18679
894 1776 1852 3262 5830 6008 7877 9570 15838 16029 16176 16583
2190 2698 3277 4748 5575 6822 8356 9692 11406 11697 12991 15275
9695 12587 15112 17987
5221 5710 15272 17606
3068 9034 11853 17189
2503 7618 9336 15768
2069 2258 7450 10219
778 8645 12173 12429
6960 9073 12411 15065
3515 5848 12776 15706
4725 5967 15682 17350
12416 14871 16503 18679
4218 13329 17613
752 6184 9180
3188 3971 11453
2580 17414 18001
10285 13728 15896
612 10652 12442
7637 7724 15724
1427 15130 15355
77 5271 8934
3121 10373 11930
11913 12253 15701
6582 9841 10243
11595 16319 16332
6402 11107 14899
4593 5442 9630
1321 3877 17467
1335 10771 12311
24 16695 18578
11396 17644 18618
7306 14777 15287
1809 5769 10827
137 3555 5186
201 3340 10470
8954 12160 17867
6744 9179 14780
3280 9637 17720
1867 10642 14613
4292 7451 14924
1621 13335 16834
8289 14826 15302
3610 12151 12159
3968 3976 5492
3491 14734 17314
3774 8427 10437
3128 4202 15889
3821 9781 10862
8264 9191 12337
1476 8123 8946

Table B.18: LDPC code identifier: 132/180 (nldpc = 64 800)

214 632 923 3251 6036 6570 8258 9462 10399 11781 12778 14807 15369 16105 17153
652 1565 3710 3720 4603 7139 7817 9076 11532 13729 14362 15379 15488 15541 15777
98 130 520 622 1806 2474 3378 4773 6896 7533 7744 11317 11511 11574 15853
95 1168 2985 4995 5032 5310 6932 8571 9181 9193 9896 10257 12336 12811 14754
1178 1969 2935 3432 3628 4814 5218 5676 6214 9953 10421 11091 13574 13772 15191
2356 7171 8062 8652 8801 9917 10037 10164 10671 10975 13460 15594 15936 16378 16711

ETSI

87

ETSI EN 302 307-2 V1.4.1 (2024-08)

1401 3622 4493 5190 6908 7193 9583 10283 11832 12152 12609 13343 13427 13839 15591
485 4930 7552 7574 7629 8514 10870 10888 11614 11774 12414 13159 15555 16874 16897
203 711 1373 5149 7271 8191 8523 9012 9645 11391 13989 14402 15572 16490 16985
1020 3606 4996 5016 7632 9959 11098 11792 12649 12859 13381 14579 16204 16899 17030
3653 4711 4777 4779 5203 8250 10671 12130 12449 13340 14148 14853
3209 4098 4415 4777 5358 6681 8049 9805 10139 15608 15628 16532
37 279 2890 3692 5680 7136 10862 11442 13688 14572 14978 16246
150 2430 2659 3909 8619 9432 12372 12720 13213 14635 15316 15727
759 7725 8548 10772 10897 11392 12273 13619 14465 14488 16191 17039
499 2346 4909 4998 6555 10631 12374 13539 13954 14728 14768 16213
286 458 1072 1982 3593 4541 5803 7260 7681 10279 15178 15701
683 850 1430 4534 4990 9870 10385 10508 12633 13516 14763 15297
1304 1620 2788 4431 8333 10080 11887 11994 12430 12578 15816 16317
1020 2376 3071 4752 7844 12085 12338 12790 13930 14874 16264 16947
2917 14555 16711
7491 9627 11576
863 2959 15686
3115 3698 4721
1992 6059 9232
6038 7185 14573
1340 3421 3694
4609 8628 12390
2208 8716 8858
13808 15922 16148
2249 11974 16896
5853 13225 13788
815 4711 6530
2209 2468 14725
4410 5415 13854
6355 6825 15280
309 9537 16469
8068 13746 14396
9323 10747 15016
6903 8218 11652
680 3121 8534
7311 10942 15810
877 965 6600
1742 5760 12311
3137 4854 11102
2422 7181 7657
11818 13570 15583
6318 13695 13717
3866 5279 6931
10864 15450 15719
4540 7389 17179
4951 15064 16397
7605 10323 11651
4137 6356 7204
5439 10310 14018
12843 13416 14274
2804 9644 10370
11150 13275 14293
5134 5240 11495
864 2151 13541
736 13561 17218
8287 13491 16780
5718 15660 16593
8455 13981 15971
9467 14810 16276
2229 3946 8111
7217 7241 12272
67 3678 5473

ETSI

88

ETSI EN 302 307-2 V1.4.1 (2024-08)

6684 10779 16599
9465 12372 16208
6794 14248 16412
2214 10815 11926
3021 6374 12487
3950 6042 9573
7939 11686 14299
350 3529 4079 4256 5849 7190 8860 10139 10232 10819 11381 14147
317 992 2421 3962 4699 6659 7506 10225 10422 10631 12471 17133
1042 1396 2353 2995 3377 5431 5872 6507 6958 8392 10521 15036
2799 3188 3338 4212 5257 6667 7299 8647 9365 9822 15393 16334
1095 1357 1964 2027 3439 5975 7077 10182 11538 12085 14873 15081
5063 15980 16044 16895
2675 3343 8369 15958
186 10209 12824 14269
4306 6720 10338 16589
2260 7944 10926 16496
821 2379 3453 11530
818 3049 7651 16046
2127 3717 10120 15916
3267 11412 13437 15833
1386 7706 15875 16377
508 11392 13620
4097 14269 15322
9921 12311 12914
7184 10571 15214
3917 8952 11193
1241 11798 14788
10457 14430 14892
5603 14302 16388
427 2770 6440
9317 10050 14671
3199 5089 5353
7239 7411 13299
306 1674 14551
816 7484 12448
706 13444 15695
554 4597 9489
2104 6359 12468
9266 10617 11381
3277 3793 6604
1731 1887 9707
885 5432 7884
1786 8137 13590
5024 6886 16155
2777 7172 8568
3551 8533 13805
3299 8732 15678
633 9789 14366
11345 14813 16179
1216 5414 13845
5832 7474 10047
1074 3156 9228
4090 7614 10391
2624 5520 13591
3462 12548 12556
2027 11569 14106
1821 3571 8001
3979 7285 9173
11161 12334 16935
2642 8811 8819
5359 11128 13310

ETSI

89

ETSI EN 302 307-2 V1.4.1 (2024-08)

200 6362 9809
1174 8836 13549

Table B.19: LDPC code identifier: 135/180 (nldpc = 64 800)

15 865 1308 2887 6202 6440 7201 9014 10015 10041 11780 13602 14265 15506
1054 1416 2903 3746 3753 7608 9121 11097 11761 12334 14304 15284 15489 15860
388 942 2207 2627 3453 6684 7105 8633 9292 9413 11574 11789 12990 13558
896 1802 2100 4497 6650 7324 7693 11232 11848 12625 12982 13238 13719 15260
2165 2313 3930 5231 9263 10942 12167 12938 13529 13806 14792 15118 15304 15970
286 951 1083 3401 5527 10235 10417 10717 12204 12522 12974 13623 13645 13721
895 2500 3051 4393 4686 5972 7932 8025 8731 9744 10323 10619 14961 16054
1631 2304 3149 3453 4133 4459 5442 7051 8622 10051 10791 11566 12754 14016
2747 4371 5647 5694 5899 8378 8965 9917 10472 12163 13349 14760 15005 16196
1119 3605 4141 4969 6694 7031 7748 8800 9268 9982 10605 11768 12185 12254
2825 3983 3991 6410 8249 8457 8770 9183 12028 12213 12448
604 1720 2373 2919 7212 7867 7967 8044 10466 13860 14417
301 1594 5664 9711 9763 10804 10816 11681 11842 12204 15041
47 555 1796 2032 3923 5175 5204 7322 12008 15192 15273
2564 2827 4053 4355 5383 6611 7951 10231 10605 12712 15035
2243 3129 5665 9703 9913 10101 10212 13549 14952 15661 15696
204 775 3771 5704 7007 7095 7543 9656 12426 12882 15545
4224 4480 4521 5860 5993 6200 6810 8966 13588 13658 14863
528 2425 4261 6534 9097 9746 10277 11570 11722 12614 14288
3612 4482 6901 8453 8546 9588 10302 11161 11365 14894 15018
3742 10567 16199
7133 9058 11953
6322 6923 15941
8088 9184 10475
677 2998 15174
4761 5594 9851
2307 13609 15098
4022 12283 12286
5993 8346 11208
3910 15175 15351
6964 10422 13372
6125 13835 14292
6234 7466 8536
4785 6567 8576
6743 10561 14130
1735 7324 11459
3414 5686 12861
5457 13085 14942
2789 9707 10189
3474 7428 8655
393 4691 5694
4825 8033 15186
1458 4367 5864
5843 11855 14660
7748 8189 15486
4810 13665 13848
5818 6651 8375
693 5872 7945
172 11594 12687
5430 12616 15658
6934 7909 11246
3637 12306 12362
3471 5213 9689
14049 14507 15642
2668 3016 15757
3740 7546 12925

ETSI

90

ETSI EN 302 307-2 V1.4.1 (2024-08)

6446 14217 15795
6834 12517 16183
6930 8193 10296
4279 5463 11460
197 1031 3531
9722 9899 11678
2962 7375 15462
181 2986 13487
908 3631 15042
3273 8070 10865
4099 6682 15571
2864 6393 12639
6486 7891 14560
10819 11213 13405
71 6734 8450
3467 5016 15956
6267 10180 15388
1625 2994 13339
2517 4489 7357
27 454 1440 1874 2627 6126 8518 9120 10144 13007 13892
439 991 5000 5256 7215 8109 8680 10694 12078 13454 15221
1162 4754 5101 5940 8304 10907 11008 11528 13514 13837 15230
1230 1618 2547 2922 5619 7415 12126 12406 14378 15306 15408
140 446 3378 3908 6904 7783 8587 10420 10630 12602 13597
1004 3374 7234 9291
8358 8550 8658 11681
3669 7500 8610 10360
4892 9971 11846 13233
329 1735 2397 13639
12658 12789 14985 15091
8580 8769 9451 15185
2383 3542 4270 8840
1379 2256 2452 15454
1457 6459 7332 12968
5323 7398 14302
6056 9938 10618
104 6041 12225
4895 14004 14522
1942 6495 6826
8262 15584 16179
11604 12644 12874
3538 9506 15206
666 6038 8853
5941 8753 12449
6500 8604 16045
7937 12018 12966
8164 14164 14528
867 6180 10192
3403 5208 10213
1752 7835 11867
1576 6993 11260
2245 8237 14506
1284 1807 5480
9778 10034 13115
8398 13975 15705
6906 7770 8242
1896 3277 10631
2168 6889 8036
1616 6908 11754
11353 13863 14389
2514 7212 12887
5661 6511 10622

ETSI

91

ETSI EN 302 307-2 V1.4.1 (2024-08)

4690 8892 10754
12200 12486 14850
4663 15405 15949
302 309 1904
5265 7100 7105
4996 7928 11084
5425 10367 15826
6766 8245 11914
8091 13882 13887
1308 1348 7944
4730 10272 14249
5001 5838 11633
3687 4732 15948
285 5437 10939
7254 10928 11235
2038 6236 14036
9407 12341 16040

Table B.20: LDPC code identifier: 140/180 (nldpc = 64 800)

66 862 939 3380 4920 5225 5330 6218 7204 7532 7689 9135 9363 10504 10694
1993 2656 4602 6079 7569 7724 9038 9647 9979 11845 12641 12783 13451 13661 14166
1360 2046 3315 3423 3974 4747 6535 6632 7261 8173 8391 9517 9928 11749 11761
3373 3910 3965 4146 4171 6195 6384 7642 9337 9563 9716 11490 12393 13068 14016
244 2500 3970 6097 6208 6669 7479 7667 8234 9367 10437 11623 12138 12212 12454
665 1162 1990 2144 2356 4400 6032 9336 9988 10693 11378 12021 12582 13874 13958
1129 1527 1725 1902 2039 2107 3241 5991 6086 7866 10793 11132 11318 13636 14100
611 2141 2552 2602 3049 3185 5339 6495 7390 8033 9068 10844 10977 11047 13995
2805 4137 4523 4841 7408 8551 8667 8749 8979 9232 9934 10345 10634 11646 12771
144 1120 2295 3469 4991 5613 7186 7858 9116 9328 10850 11492 11837 12155 13103
803 1580 1797 4719 6743 7061 7753 8376 9046 11635 11721 13350
1432 3534 4822 6282 6412 7180 7619 7936 11278 12531 13074 13084
2013 2575 2887 3930 4725 5498 5625 6209 6836 7268 9062 10950
515 1037 2033 2624 3044 6028 7163 8729 8772 10358 10659 12466
464 1685 2749 3321 3778 5322 5386 6294 7928 8871 10278 13040
408 829 1672 2667 3077 3545 3703 5213 5381 7937 8474 13126
1617 2490 2636 2723 5431 6975 7159 7900 10849 11572 11887 12462
1402 2373 6408 6656 6704 8040 8841 9541 11818 13891 14006 14239
1388 2078 2136 3514 5090 8083 8510 9200 9814 11142 11625 12980
561 1659 2611 3085 3367 3804 6021 6209 6348 8282 8475 11386
2457 3223 4495 4869 5314 5774 6532 6552 8987 9196 9199 11591
627 1069 3015 3048 4275 4545 4617 5606 6070 8237 8659 8953
1028 4096 5253 6370 8087 8382 8950 8984 9618 12843 13519 14356
560 604 663 2209 2709 4421 6291 7322 10054 11747 11997 14192
361 938 993 2884 3386 9431 9798 10155 11892 12184 13140 13808
1045 5017 9862 13620
205 3913 9136 13316
2994 4307 10330 13439
2437 6367 9411 10505
5546 6653 7663 12391
2825 3446 5803 11254
1459 5988 7895 9221
3968 6470 7739 12977
3298 4154 12918 14282
8890 9389 10144 12801
2529 3668 10005 11652
4558 8324 10112 12696
491 6153 11815 12813
1300 12716 13107 13847
5542 6160 11226 12846
5206 9994 11133

ETSI

92

ETSI EN 302 307-2 V1.4.1 (2024-08)

7113 12088 12802
950 1805 8437
4411 10474 12911
3599 7495 8984
4751 10097 10284
67 5056 11979
10633 10770 13585
1198 3963 9181
746 4895 11288
7724 8170 9246
6542 8235 8589
1512 4603 11098
7082 13053 13985
3887 9094 10355
3417 6588 12800
4151 5569 8184
5557 8162 12572
2565 6091 6359
2792 4430 6281
7936 10786 11229
677 3184 12460
2468 2884 11714
87 2318 9383
976 3614 10292
969 3180 14286
7818 12285 13535
3471 10797 11502
3552 10568 12836
1386 6971 13457
987 7598 9275
5039 13533 13739
1854 5210 11261
10603 11753 12263
722 1030 12267
2720 5083 5350 9274
3377 8717 9414 12039
1355 6452 10130 13008
5112 7583 9971 10955
4633 8781 12791 13607
1535 5803 8062 10467
2326 8224 9176 12082
939 8480 11823 13045
380 425 4943 10388
4001 4293 7887 9858
3734 3746 9929 12310
1592 6062 6419 10292
101 2538 6316 13640
3038 5921 6156 6529
3820 10279 12229 12404
761 3735 8874
4985 9636 14004
1744 2501 9257
3223 7816 10249
765 2768 5363
4911 5948 13726
6745 9749 11658
1373 4860 13952
120 407 13132
862 2571 3681
3706 5914 8019
7465 10479 12795
441 1017 1563

ETSI

93

ETSI EN 302 307-2 V1.4.1 (2024-08)

6638 8730 10379
3229 4169 11333
1181 7252 11670
1090 4576 8680
943 9116 11566
3180 7882 12535
2944 4411 12747
3153 5618 7782
428 2208 10359
447 6906 12192
8495 11164 12870
641 6397 11868
4165 4534 11544
4594 7957 11969
3667 4604 7920
2253 4617 13638
1099 4214 6076
461 8085 9875
8128 13331 13740
8527 9749 12563
4216 6105 12391
6583 13362 14130
566 2898 4772
4048 7696 8342
539 5111 9923
931 3789 7276
5306 13717 13901
1540 11240 11353
1845 2752 6810
8553 10094 10228
9625 12761 13252
4518 4526 9215
5394 6447 10864
7497 11962 12795
2679 3310 3743
2764 10853 12702
6409 9498 10387

Table B.21: LDPC code identifier: 154/180 (nldpc = 64 800)

726 794 1587 2475 3114 3917 4471 6207 7451 8203 8218 8583 8941
418 480 1320 1357 1481 2323 3677 5112 7038 7198 8066 9260 9282
1506 2585 3336 4543 4828 5571 5954 6047 6081 7691 8090 8824 9153
164 888 1867 2685 2983 4071 4848 4930 5882 7085 7861 8545 8689
766 1004 1143 1782 1996 2506 2944 3094 4085 5728 8634 8737 8759
199 341 2068 2100 2708 2896 4173 5846 6167 7798 9101 9159 9329
617 752 1647 2258 2597 4708 5808 6726 7293 7800 7988 8825 9055
315 408 620 1256 1985 2560 3226 5526 6463 6927 7223 7505 7669
1361 1528 2397 3246 3370 4333 5167 5333 7409 8075 8197 8279 9138
665 876 2039 2703 2864 3485 3767 4823 7275 7599 8274 8694 9334
1160 1717 1750 2158 3040 3506 3764 3828 4261 4292 5134 6789
1857 2119 2952 6145 6897 7582 7639 8032 8080 8181 8221 8454
421 794 1511 2166 2489 3936 4288 5440 5901 6490 7266 8858
456 2565 4071 4395 4451 4888 5338 5351 6608 7602 7835 9024
617 912 3362 4085 4404 5050 5244 6043 6444 6721 7414 8353
4535 7597 7853
2632 4652 6491
877 1378 8828
434 3309 8607
1075 2525 4103
958 2220 3471

ETSI

94

ETSI EN 302 307-2 V1.4.1 (2024-08)

2256 4350 7277
1731 4306 8524
470 6369 9026
2605 3171 8760
1886 4900 7558
3825 4488 9230
228 3806 8101
3607 7835 8035
5280 7413 8380
2606 5100 5549
2309 4329 8570
1577 4746 7473
2939 4664 7327
2440 8572 8912
4143 8221 8561
3982 5316 7329
387 745 5199
2563 4077 9076
1428 6482 9178
4600 7640 8483
3918 5239 5388
2006 6768 9041
5766 7058 7758
2741 3382 5713
116 1863 4193
2339 4499 8437
1799 5408 6711
6937 7536 8100
8313 8388 9277
1743 3100 7424
1959 2130 2230
5007 6692 7748
808 3333 5951
1719 7648 8645
102 2996 6153
739 2765 6496
1107 1760 7023
1067 2666 9235
1125 3760 8095
2047 3143 6383
2698 3440 5405
1746 1854 1965
380 3171 5816
4155 5210 9325
2290 2658 3766
167 6239 6635
1338 1541 5813
6148 6574 7436
3598 3777 6367
731 4247 8887
2152 2600 8950
3774 4099 6605
2819 3315 6492
1195 3774 7126
572 2723 3467 3509 5379 6756 6853 7335 7973 8087 8202 9000
817 3136 3533 3732 5001 5148 5202 5402 6602 7236 7605 8275
185 325 591 1559 1635 2826 3352 3634 3937 5814 8101 8133
758 1780 1965 2932 4010 4621 7103 7249 7328 7878 8754 8805
528 1433 2926 3557 3780 4650 4671 5253 5488 5517 5681 8300
1172 2131 3702 4455 4504 5216 5920 6371 6660 7953 9036 9185
639 1572 1714 1957 3145 5123 5330 5419 6418 7163 7237 9235
166 486 708 1071 2179 3700 4756 5606 5685 6426 6467 6902

ETSI

95

ETSI EN 302 307-2 V1.4.1 (2024-08)

462 486 735 2065 2558 3196 4006 5344 5617 7451 8141 8436
435 3016 4173 4235 4415 4731 5396 7340 8002 8155 8279 9081
560 2200 2649 3690 8636
4156 5971 7133 7480 8218
1398 2219 3796 4877 6376
506 1284 6906 7288 9131
643 1661 5057 8011 8241
859 3508 5030
575 3942 6198
3472 5037 8710
3850 8080 9216
3203 8128 8836
3059 5057 8120
3804 6339 8539
2355 6922 8235
2035 2133 7090
4787 5994 6966
1484 4897 7766
3977 7211 7682
3030 4150 7823
516 2443 7919
5120 5715 6141
1551 6029 7124
1995 2156 6952
4686 4944 8175
2763 4667 7284
3648 7312 7361
333 3231 4029
692 2273 9068
15 3757 7203
2870 4217 8458
1501 2721 6548
92 2144 6570
1846 4578 7972
2999 3542 4001
1658 8479 8763
4169 6305 7774
2357 2475 8504
1418 1516 3587
2715 2754 7789
1765 2387 8858
5115 8712 9029
160 2544 5818
1600 3668 7706
1589 3143 7396
3310 3953 8862
2054 3075 4821
4061 4355 6130
2086 2534 4831
4229 4981 9057
24 5398 6062
1370 7446 8116
409 1199 6499
1088 1648 7267
176 8059 9351
558 3830 4748
4772 8116 8277
1253 2418 3450
5305 5679 7537
437 561 7932
3058 4317 9184
382 1516 6576

ETSI

96

ETSI EN 302 307-2 V1.4.1 (2024-08)

471 6158 7469
5 955 2716
964 5239 8890
727 738 4868
7443 7560 7580
2075 2266 8918
4021 4267 6797
6103 6111 8823
6523 6531 9063

Table B.22: LDPC code identifier: 18/30 (nldpc = 64 800)

113 1557 3316 5680 6241 10407 13404 13947 14040 14353 15522 15698 16079 17363 19374 19543 20530 22833
24339
271 1361 6236 7006 7307 7333 12768 15441 15568 17923 18341 20321 21502 22023 23938 25351 25590 25876
25910
73 605 872 4008 6279 7653 10346 10799 12482 12935 13604 15909 16526 19782 20506 22804 23629 24859 25600
1445 1690 4304 4851 8919 9176 9252 13783 16076 16675 17274 18806 18882 20819 21958 22451 23869 23999
24177
1290 2337 5661 6371 8996 10102 10941 11360 12242 14918 16808 20571 23374 24046 25045 25060 25662 25783
25913
28 42 1926 3421 3503 8558 9453 10168 15820 17473 19571 19685 22790 23336 23367 23890 24061 25657 25680
0 1709 4041 4932 5968 7123 8430 9564 10596 11026 14761 19484 20762 20858 23803 24016 24795 25853 25863
29 1625 6500 6609 16831 18517 18568 18738 19387 20159 20544 21603 21941 24137 24269 24416 24803 25154
25395
55 66 871 3700 11426 13221 15001 16367 17601 18380 22796 23488 23938 25476 25635 25678 25807 25857 25872
1 19 5958 8548 8860 11489 16845 18450 18469 19496 20190 23173 25262 25566 25668 25679 25858 25888 25915
7520 7690 8855 9183 14654 16695 17121 17854 18083 18428 19633 20470 20736 21720 22335 23273 25083 25293
25403
48 58 410 1299 3786 10668 18523 18963 20864 22106 22308 23033 23107 23128 23990 24286 24409 24595 25802
12 51 3894 6539 8276 10885 11644 12777 13427 14039 15954 17078 19053 20537 22863 24521 25087 25463 25838
3509 8748 9581 11509 15884 16230 17583 19264 20900 21001 21310 22547 22756 22959 24768 24814 25594 25626
25880
21 29 69 1448 2386 4601 6626 6667 10242 13141 13852 14137 18640 19951 22449 23454 24431 25512 25814
18 53 7890 9934 10063 16728 19040 19809 20825 21522 21800 23582 24556 25031 25547 25562 25733 25789 25906
4096 4582 5766 5894 6517 10027 12182 13247 15207 17041 18958 20133 20503 22228 24332 24613 25689 25855
25883
0 25 819 5539 7076 7536 7695 9532 13668 15051 17683 19665 20253 21996 24136 24890 25758 25784 25807
34 40 44 4215 6076 7427 7965 8777 11017 15593 19542 22202 22973 23397 23423 24418 24873 25107 25644
1595 6216 22850 25439
1562 15172 19517 22362
7508 12879 24324 24496
6298 15819 16757 18721
11173 15175 19966 21195
59 13505 16941 23793
2267 4830 12023 20587
8827 9278 13072 16664
14419 17463 23398 25348
6112 16534 20423 22698
493 8914 21103 24799
6896 12761 13206 25873
2 1380 12322 21701
11600 21306 25753 25790
8421 13076 14271 15401
9630 14112 19017 20955
212 13932 21781 25824
5961 9110 16654 19636
58 5434 9936 12770
6575 11433 19798
2731 7338 20926
14253 18463 25404

ETSI

97

ETSI EN 302 307-2 V1.4.1 (2024-08)

21791 24805 25869
2 11646 15850
6075 8586 23819
18435 22093 24852
2103 2368 11704
10925 17402 18232
9062 25061 25674
18497 20853 23404
18606 19364 19551
7 1022 25543
6744 15481 25868
9081 17305 25164
8 23701 25883
9680 19955 22848
56 4564 19121
5595 15086 25892
3174 17127 23183
19397 19817 20275
12561 24571 25825
7111 9889 25865
19104 20189 21851
549 9686 25548
6586 20325 25906
3224 20710 21637
641 15215 25754
13484 23729 25818
2043 7493 24246
16860 25230 25768
22047 24200 24902
9391 18040 19499
7855 24336 25069
23834 25570 25852
1977 8800 25756
6671 21772 25859
3279 6710 24444
24099 25117 25820
5553 12306 25915
48 11107 23907
10832 11974 25773
2223 17905 25484
16782 17135 20446
475 2861 3457
16218 22449 24362
11716 22200 25897
8315 15009 22633
13 20480 25852
12352 18658 25687
3681 14794 23703
30 24531 25846
4103 22077 24107
23837 25622 25812
3627 13387 25839
908 5367 19388
0 6894 25795
20322 23546 25181
8178 25260 25437
2449 13244 22565
31 18928 22741
1312 5134 14838
6085 13937 24220
66 14633 25670
47 22512 25472

ETSI

98

ETSI EN 302 307-2 V1.4.1 (2024-08)

8867 24704 25279
6742 21623 22745
147 9948 24178
8522 24261 24307
19202 22406 24609

Table B.23: LDPC code identifier: 20/30 (nldpc = 64 800)

692 1779 1973 2726 5151 6088 7921 9618 11804 13043 15975 16214 16889 16980 18585 18648
13 4090 4319 5288 8102 10110 10481 10527 10953 11185 12069 13177 14217 15963 17661 20959
2330 2516 2902 4087 6338 8015 8638 9436 10294 10843 11802 12304 12371 14095 18486 18996
125 586 5137 5701 6432 6500 8131 8327 10488 11032 11334 11449 12504 16000 20753 21317
30 480 2681 3635 3898 4058 12803 14734 20252 20306 20680 21329 21333 21466 21562 21568
20 44 738 4965 5516 7659 8464 8759 12216 14630 18241 18711 19093 20217 21316 21490
31 43 3554 5289 5667 8687 14885 16579 17883 18384 18486 19142 20785 20932 21131 21308
7054 9276 10435 12324 12354 13849 14285 16482 19212 19217 19221 20499 20831 20925 21195 21247
9 13 4099 10353 10747 14884 15492 17650 19291 19394 20356 20658 21068 21117 21183 21586
28 2250 2980 8988 10282 12503 13301 18351 20546 20622 21006 21293 21344 21472 21530 21542
17 32 2521 4374 5098 7525 13035 14437 15283 18635 19136 20240 21147 21179 21300 21349
57 4735 5657 7649 8807 12375 16092 16178 16379 17545 19461 19489 20321 20530 21453 21457
35 55 5333 14423 14670 15438 19468 19667 20823 21084 21241 21344 21447 21520 21554 21586
13 20 2025 11854 12516 14938 15929 18081 19730 19929 20408 21338 21391 21425 21468 21546
54 7451 8176 10136 15240 16442 16482 19431 19483 19762 20647 20839 20966 21512 21579 21592
26 465 3604 4233 9831 11741 13692 18953 18974 21021 21039 21133 21282 21488 21532 21558
1 7 16 59 6979 7675 7717 9791 12370 13050 18534 18729 19846 19864 20127 20165
15 31 11089 12360 13640 14237 17937 18043 18410 19443 21107 21444 21449 21528 21576 21584
32 51 9768 17848 18095 19326 19594 19618 19765 20440 20482 20582 21236 21338 21563 21587
44 55 4864 10253 11306 12117 13076 13901 15610 17057 18205 19794 20939 21132 21267 21573
3436 11304 15361 16511 16860 18238 18639 19341 20106 20123 20407 21200 21280 21452 21526 21569
679 8822 11045 14403 16588 17838 19117 19453 20265 20558 21374 21396 21428 21442 21529 21590
391 13002 13140 14314 17169 17175 17846 18122 19447 20075 20212 20436 20583 21330 21359 21403
7601 10257 20060 21285
4419 9150 18097 20315
4675 13376 21435
610 1238 16704
5732 7096 21104
5690 13531 14545
4334 14839 17357
8 2814 17674
2392 8128 18369
502 7403 15133
343 13624 20673
13188 15687 21593
321 16866 21347
1242 4261 17449
4691 8086 8691
8500 11538 20278
6269 12905 18192
5984 15452 17111
11541 18717 21534
16 10780 16107
12310 12959 20390
1365 18306 19634
6125 19132 20242
3012 17233 21533
5816 13021 21440
13207 17811 18798
2762 7586 12139
3949 5545 13584
11374 18279 19241
2736 10989 21209

ETSI

99

ETSI EN 302 307-2 V1.4.1 (2024-08)

4095 20677 21395
8251 10084 20498
7628 8875 21406
2743 8943 9090
1817 7788 15767
9333 9838 21268
6203 9480 12042
5747 21187 21468
2553 18281 21500
3179 9155 15222
12498 18109 20326
14106 21209 21592
7454 17484 20791
20804 21120 21574
5754 18178 20935
30 4322 21381
11905 20416 21397
12452 19899 21497
1917 6028 16868
9891 18710 18953
912 21083 21446
370 14355 18069
16519 19003 20902
11163 17558 18424
8427 14396 21405
8885 11796 21361
4960 15431 20653
11944 16839 21236
9967 14529 17208
14144 19354 19745
7986 12680 21396
6097 11501 13028
33 13803 21038
3177 20124 20803
2692 6841 18655
971 5892 14354
3887 19455 21271
17214 17315 21148
6539 13910 21526
3809 5153 15793
3865 21438 21510
7129 17787 19636
5972 13150 14182
7078 14906 16911
15705 21160 21482
5479 13860 19763
16817 19722 20001
14649 16147 18886
15138 18578 21502
2096 2534 17760
11920 13460 19783
19876 20071 20583
6241 14230 20775
16138 16386 21371
8616 15624 18453
6013 8015 21599
9184 10688 20792
18122 21141 21469
10706 13177 20957
15148 15584 20959
9114 9432 16467
5483 14687 14705

ETSI

100

ETSI EN 302 307-2 V1.4.1 (2024-08)

8325 21161 21410
2328 17670 19834
7015 20802 21385
52 5451 20379
9689 15537 19733

Table B.24: LDPC code identifier: 22/30 (nldpc = 64 800)

696 989 1238 3091 3116 3738 4269 6406 7033 8048 9157 10254 12033 16456 16912
444 1488 6541 8626 10735 12447 13111 13706 14135 15195 15947 16453 16916 17137 17268
401 460 992 1145 1576 1678 2238 2320 4280 6770 10027 12486 15363 16714 17157
1161 3108 3727 4508 5092 5348 5582 7727 11793 12515 12917 13362 14247 16717 17205
542 1190 6883 7911 8349 8835 10489 11631 14195 15009 15454 15482 16632 17040 17063
17 487 776 880 5077 6172 9771 11446 12798 16016 16109 16171 17087 17132 17226
1337 3275 3462 4229 9246 10180 10845 10866 12250 13633 14482 16024 16812 17186 17241
15 980 2305 3674 5971 8224 11499 11752 11770 12897 14082 14836 15311 16391 17209
0 3926 5869 8696 9351 9391 11371 14052 14172 14636 14974 16619 16961 17033 17237
3033 5317 6501 8579 10698 12168 12966 14019 15392 15806 15991 16493 16690 17062 17090
981 1205 4400 6410 11003 13319 13405 14695 15846 16297 16492 16563 16616 16862 16953
1725 4276 8869 9588 14062 14486 15474 15548 16300 16432 17042 17050 17060 17175 17273
1807 5921 9960 10011 14305 14490 14872 15852 16054 16061 16306 16799 16833 17136 17262
2826 4752 6017 6540 7016 8201 14245 14419 14716 15983 16569 16652 17171 17179 17247
1662 2516 3345 5229 8086 9686 11456 12210 14595 15808 16011 16421 16825 17112 17195
2890 4821 5987 7226 8823 9869 12468 14694 15352 15805 16075 16462 17102 17251 17263
3751 3890 4382 5720 10281 10411 11350 12721 13121 14127 14980 15202 15335 16735 17123
26 30 2805 5457 6630 7188 7477 7556 11065 16608 16859 16909 16943 17030 17103
40 4524 5043 5566 9645 10204 10282 11696 13080 14837 15607 16274 17034 17225 17266
904 3157 6284 7151 7984 11712 12887 13767 15547 16099 16753 16829 17044 17250 17259
7 311 4876 8334 9249 11267 14072 14559 15003 15235 15686 16331 17177 17238 17253
4410 8066 8596 9631 10369 11249 12610 15769 16791 16960 17018 17037 17062 17165 17204
24 8261 9691 10138 11607 12782 12786 13424 13933 15262 15795 16476 17084 17193 17220
88 11622 14705 15890
304 2026 2638 6018
1163 4268 11620 17232
9701 11785 14463 17260
4118 10952 12224 17006
3647 10823 11521 12060
1717 3753 9199 11642
2187 14280 17220
14787 16903 17061
381 3534 4294
3149 6947 8323
12562 16724 16881
7289 9997 15306
5615 13152 17260
5666 16926 17027
4190 7798 16831
4778 10629 17180
10001 13884 15453
6 2237 8203
7831 15144 15160
9186 17204 17243
9435 17168 17237
42 5701 17159
7812 14259 15715
39 4513 6658
38 9368 11273
1119 4785 17182
5620 16521 16729
16 6685 17242
210 3452 12383

ETSI

101

ETSI EN 302 307-2 V1.4.1 (2024-08)

466 14462 16250
10548 12633 13962
1452 6005 16453
22 4120 13684
5195 11563 16522
5518 16705 17201
12233 14552 15471
6067 13440 17248
8660 8967 17061
8673 12176 15051
5959 15767 16541
3244 12109 12414
31 15913 16323
3270 15686 16653
24 7346 14675
12 1531 8740
6228 7565 16667
16936 17122 17162
4868 8451 13183
3714 4451 16919
11313 13801 17132
17070 17191 17242
1911 11201 17186
14 17190 17254
11760 16008 16832
14543 17033 17278
16129 16765 17155
6891 15561 17007
12741 14744 17116
8992 16661 17277
1861 11130 16742
4822 13331 16192
13281 14027 14989
38 14887 17141
10698 13452 15674
4 2539 16877
857 17170 17249
11449 11906 12867
285 14118 16831
15191 17214 17242
39 728 16915
2469 12969 15579
16644 17151 17164
2592 8280 10448
9236 12431 17173
9064 16892 17233
4526 16146 17038
31 2116 16083
15837 16951 17031
5362 8382 16618
6137 13199 17221
2841 15068 17068
24 3620 17003
9880 15718 16764
1784 10240 17209
2731 10293 10846
3121 8723 16598
8563 15662 17088
13 1167 14676
29 13850 15963
3654 7553 8114
23 4362 14865

ETSI

102

ETSI EN 302 307-2 V1.4.1 (2024-08)

4434 14741 16688
8362 13901 17244
13687 16736 17232
46 4229 13394
13169 16383 16972
16031 16681 16952
3384 9894 12580
9841 14414 16165
5013 17099 17115
2130 8941 17266
6907 15428 17241
16 1860 17235
2151 16014 16643
14954 15958 17222
3969 8419 15116
31 15593 16984
11514 16605 17255

ETSI

103

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex C (normative):
Addresses of parity bit accumulators for nldpc = 16 200 and
nldpc = 32 400

Table C.1: LDPC code identifier: 11/45 (nldpc = 16 200)

9054 9186 12155 1000 7383 6459 2992 4723 8135 11250
2624 9237 7139 12238 11962 4361 5292 10967 11036 8105
2044 11996 5654 7568 7002 3549 4767 8767 2872 8345
6966 8473 5180 8084 3359 5051 9576 5139 1893 902
3041 3801 8252 11951 909 8535 1038 8400 3200 4585
5291 10484 10872
442 7516 3720
11469 769 10998
10575 1436 2935
6905 8610 11285
1873 5634 6383

Table C.2: LDPC code identifier: 4/15 (nldpc = 16 200)

1953 2331 2545 2623 4653 5012 5700 6458 6875 7605 7694 7881 8416 8758 9181 9555 9578 9932 10068 11479
11699
514 784 2059 2129 2386 2454 3396 5184 6624 6825 7533 7861 9116 9473 9601 10432 11011 11159 11378 11528
11598
483 1303 1735 2291 3302 3648 4222 4522 5511 6626 6804 7404 7752 7982 8108 8930 9151 9793 9876 10786 11879
1956 7572 9020 9971
13 1578 7445 8373
6805 6857 8615 11179
7983 8022 10017 11748
4939 8861 10444 11661
2278 3733 6265 10009
4494 7974 10649
8909 11030 11696
3131 9964 10480

Table C.3: LDPC code identifier: 14/45 (nldpc = 16 200)

1606 3617 7973 6737 9495 4209 9209 4565 4250 7823 9384 400
4105 991 923 3562 3892 10993 5640 8196 6652 4653 9116 7677
6348 1341 5445 1494 7799 831 4952 5106 3011 9921 6537 8476
7854 5274 8572 3741 5674 11128 4097 1398 5671 7302 8155 2641
6548 2103 590 5749 5722 10 2682 1063 633 2949 207 6065
2828 6366 4766 399 935 7611 84 150 3146
5363 7455 7140
9297 482 4848
8458 1631 5344
5729 6767 4836
11019 4463 3882
4107 9610 5454
11137 4328 6307
3260 7897 3809

ETSI

104

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table C.4: LDPC code identifier: 7/15 (nldpc = 16 200)

3 137 314 327 983 1597 2028 3043 3217 4109 6020 6178 6535 6560 7146 7180 7408 7790 7893 8123 8313 8526 8616
8638
356 1197 1208 1839 1903 2712 3088 3537 4091 4301 4919 5068 6025 6195 6324 6378 6686 6829 7558 7745 8042
8382 8587 8602
18 187 1115 1417 1463 2300 2328 3502 3805 4677 4827 5551 5968 6394 6412 6753 7169 7524 7695 7976 8069 8118
8522 8582
714 2713 2726 2964 3055 3220 3334 3459 5557 5765 5841 6290 6419 6573 6856 7786 7937 8156 8286 8327 8384
8448 8539 8559
3452 7935 8092 8623
56 1955 3000 8242
1809 4094 7991 8489
2220 6455 7849 8548
1006 2576 3247 6976
2177 6048 7795 8295
1413 2595 7446 8594
2101 3714 7541 8531
10 5961 7484
3144 4636 5282
5708 5875 8390
3322 5223 7975
197 4653 8283
598 5393 8624
906 7249 7542
1223 2148 8195
976 2001 5005

Table C.5: LDPC code identifier: 8/15 (nldpc = 16 200)

32 384 430 591 1296 1976 1999 2137 2175 3638 4214 4304 4486 4662 4999 5174 5700 6969 7115 7138 7189
1788 1881 1910 2724 4504 4928 4973 5616 5686 5718 5846 6523 6893 6994 7074 7100 7277 7399 7476 7480 7537
2791 2824 2927 4196 4298 4800 4948 5361 5401 5688 5818 5862 5969 6029 6244 6645 6962 7203 7302 7454 7534
574 1461 1826 2056 2069 2387 2794 3349 3366 4951 5826 5834 5903 6640 6762 6786 6859 7043 7418 7431 7554
14 178 675 823 890 930 1209 1311 2898 4339 4600 5203 6485 6549 6970 7208 7218 7298 7454 7457 7462
4075 4188 7313 7553
5145 6018 7148 7507
3198 4858 6983 7033
3170 5126 5625 6901
2839 6093 7071 7450
11 3735 5413
2497 5400 7238
2067 5172 5714
1889 7173 7329
1795 2773 3499
2695 2944 6735
3221 4625 5897
1690 6122 6816
5013 6839 7358
1601 6849 7415
2180 7389 7543
2121 6838 7054
1948 3109 5046
272 1015 7464

ETSI

105

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table C.6: LDPC code identifier: 26/45 (nldpc = 16 200)

6106 5389 698 6749 6294 1653 1984 2167 6139 6095 3832 2468 6115
4202 2362 1852 1264 3564 6345 498 6137 3908 3302 527 2767 6667
3422 1242 1377 2238 2899 1974 1957 261 3463 4994 215 2338
3016 5109 6533 2665 5300 4908 4967 5787 726 229 1970 2789
6146 5765 6649 2871 884 1670 2597 5058 3659 6594 5042 304
5521 2811 0 4214 2626 2211 1236 3771 852 6356 6797 3463
1523 1830 3938 5593 2128 5791 3421 3680 6692 1377 3808 3475
5551 6035 2247 3662 759 6783 116 6380 4586 3367 1 5003
3518 6557 6510
1830 839 4421
5431 5959 6152
3174 5113 4520
5399 1303 2496
2841 741 220
2731 1830 4193
1875 3935 223
9 4720 423
3107 2676 840
1950 6177 6457
4091 94 5102
1907 6050 3455
714 3 559
502 4268 4164
1019 5558 271
6127 854 3221
959 5337 2735

Table C.7: LDPC code identifier: 32/45 (nldpc = 16 200)

2686 655 2308 1603 336 1743 2778 1263 3555 185 4212 621
286 2994 2599 2265 126 314 3992 4560 2845 2764 2540 1476
2670 3599 2900 2281 3597 2768 4423 2805 836 130 1204 4162
1884 4228 1253 2578 3053 3650 2587 4468 2784 1644 1490 4655
4258 1699 4363 4555 3810 4046 3806 344 2459 4067 3327 3510
1021 2741 2528 2168 2820
254 1080 616
1465 4192 2972
2356 2976 1534
4412 1937 2724
1430 3024 600
1952 2136 3573
3009 3123 1288
4553 2299 806
2997 402 4330
3302 4567 698
2364 498 3146
1809 647 992
3512 32 4301
1238 251 450
1657 737 641
560 1720 2893
1689 2206 902
3998 1784 2094
2090 3126 1201
1565 764 3473
891 903 2413
2286 2900 2348
3026 2033 1502
2404 1243 556

ETSI

106

ETSI EN 302 307-2 V1.4.1 (2024-08)

308 2222 3825
1523 3311 389

Table C.8: LDPC code identifier: 1/5 (nldpc = 32 400)

18222 6715 4908 21568 22821 11708 4769 4495 22243 25872 9051 19072 13956
2038 5205 21215 21009 9584 2403 23652 20866 20130 677 9509 6136 773
19936 14590 17829 473 4432 23171 11386 17937 22084 24450 267 8822 19335
16376 16769 5111 9794 18907 827 12385 12370 21647 10938 23619 11633 15865
23417 7631 12243 21546 4192 22117 14757 4118 9686 17021 8531 15989 8807
15533 16584 18529 19699 17821 4252 1254 5952 3163 20295 6944 1022 19743
129 16579 23524 25897 14690 11222 16250 9925 4268 999 7102 24528 152
18361 3708 3454 16604 1551 5809 20324 4775 22418 19091 19674 10975 7327
24133 10950 22779 11388 13818 20668 7556 12333 16446 19684 12510 25118 8162
17026 6850 1269
21895 7137 25270
11858 24153 13303
7885 16438 12805
10473 15004 8052
2088 10379 10067
21438 13426 10440
17696 727 12164
22623 8408 17849

Table C.9: LDPC code identifier: 11/45 (nldpc = 32 400)

20617 6867 14845 11974 22563 190 17207 4052 7406 16007
21448 14846 2543 23380 16633 20365 16869 13411 19853 795
5200 2330 2775 23620 20643 10745 14742 6493 14222 20939
9445 9523 12769 7332 21792 18717 16397 14016 9481 22162
2922 6427 4497 4116 17658 2581 14364 3781 18851 22974
10383 2184 1433 3889 12828 17424 17580 20936 1390 21374
425 2063 22398 20907 9445 14790 4457 723 7048 4072
11771 9640 23212 9613 12042 8335 21386 20129 13521 16301
14867 12501 1086 21526 17701 17731 20907 8790 19224 5784
7107 19690 17616 5800 9501 23320 16878 794 15931 17539
4556 21783 1524
20100 11706 23663
2535 15530 6116
12078 3867 2663
19629 20246 7024
11748 11426 19802
15942 12333 5316
11521 3170 17818
2289 23780 16575
6649 16991 13025
20050 10619 10250
3944 13063 5656

ETSI

107

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table C.10: LDPC code identifier: 1/3 (nldpc = 32 400)

7416 4093 16722 1023 20586 12219 9175 16284 1554 10113 19849 17545
13140 3257 2110 13888 3023 1537 1598 15018 18931 13905 10617 1014
339 14366 3309 15360 18358 3196 4412 6023 7070 17380 2777 6691
12720 17634 4141 1400 8247 18201 16077 11314 11928 3494 3155 2865
21038 6928 3860 1943 20292 6526 12939 15182 3957 5651 356 2673
20555 17905 5724 13932 1218 17763 5912 5164 6233 6220 1277 19209
19190 4498 4950 6645 5482 5667 13701 16334 15231 735 8589 12344
679 17849 17807 16033 10181 3368 5778 8275 2736 14042 17506 6005
1576 10259 10525 3007 16522 697 7726 8641 14323 2893 8168 11070
17270 10180 18740 847 4969 14715 19316 5530 17428 11193 9861 13562
6156 18787 10467
2422 3723 10769
8015 18716 13406
5969 15949 3084
6855 13990 3764
10351 15779 10392
16078 19034 11279
11747 6608 4188
19699 8928 8045
4598 7219 11391
19766 11871 5692
7487 15905 17621
8554 7751 16516
4981 20250 16146
12524 21364 10793
17083 2051 8862
1315 6246 7721
18043 16652 5502
1432 5674 2224
11257 1312 8453

ETSI

108

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex D (normative):
Additional tools

D.0  General aspects

See ETSI EN 302 307-1 [3], Annex D.

D.1

Implementation of TS based channel bonding

D.1.1  Transmitting side

The L branches output L partial Transport-Streams, each with exactly the same bit-rate of the input "big-TS", but with a
variable density of added null-packets (NP in Figure 2). The SI tables are copied in all branches in order to allow a
decoder to discover, during frequency scanning, sets of bonded transponders; therefore, to avoid buffer overflow, the
available net capacity (excluding null-packets, which are not transmitted) of the L channels shall slightly exceed the
capacity of the big-TS. Differently from S2, in the channel-bonding mode, Input Stream Synchronization, Null-packet
deletion and Dummy Frame insertion shall be active, although each S2X modulator is set to Single-Transport Stream
mode, for broadcast services. The master channel, used for ISSY reference, should be robust enough to minimize loss of
time resynchronization at receiver side. It shall further have a symbol clock rate allowing sufficiently fine temporal
resolution. The useful packet interval shall follow the above description. However, one BBFRAME delay can be
tolerated in addition between the different modulators. Original Null Packets in the "big-Transport-Stream" are either
deleted in NPD or transmitted in the same manner as useful packets (incl. ISSY insertion). In case of multiple-input
stream mode TS, some PIDs may be transmitted over a single transponder, while others use channel bonding over L
transponders. In such a case, these "single-transponder PIDs" shall not be part of the "big-Transport-Stream", but
directed to a specific transponder. Their rate shall thus be ignored in the above formula of the useful packet interval (in
the same was as PIDs ∈{SI tables} are excluded from this rate). Bonded channels shall be in located in the same
frequency band.

D.1.2  Receiving side (informative)

Services are spread over the various branches, therefore it is not sufficient to receive a single partial TS to decode an
audio, video or data service and a multiple receiver has to be adopted, with L demodulators working in parallel to
reconstruct the L partial transport streams (by re-inserting the deleted null-packets). By means of L FIFO buffers (the
dimension of which are dependent on the difference between satellite channel delays, which shall not exceed 200 µs)
and the information of the ISSY fields, a multiple receiver may re-align the L partial Transport-Streams. After
re-alignment, such a receiver may exactly reconstruct the original "big-TS" by merging the partial TSs from the L
branches (i.e. when a useful-packet is present in a branch, and null-packets in the other L-1 branches, the useful-packet
is retained; when null-packets or equal SI packets are present in all the L branches, such packet is retained). The output
clock of the "big-TS" can be reconstructed as shown in clause D.2 of ETSI EN 302 307-1 [3], from the recovered
symbol-clock of Modulator 1 and the ISSY field time-stamps. In case original Null Packets (from "big-Transport
Stream") are transmitted as useful packets, the corresponding input to the MERGE block at receiver side will be Null
Packets in all branches. In such a case, the receiver shall select any branch, e.g. branch number 1.

D.2  Void

D.3  Void

ETSI

D.4  Void

109

ETSI EN 302 307-2 V1.4.1 (2024-08)

D.5  Signalling of reception quality via return channel

(normative for ACM)

In ACM modes, the receiver shall signal the reception quality via an available return channel, according to the various
DVB interactive systems, such as for example:

•

•

•

•

•

DVB-RCS (ETSI EN 301 790 [11]

DVB-RCS2 (ETSI TS 101 545-1 [1])

DVB-RCP (ETSI ETS 300 801 [7])

DVB-RCG (ETSI EN 301 195 [8])

DVB-RCC (ETSI ES 200 800 [9])

DVB "Network Independent Protocols for DVB Interactive Services" (ETSI ETS 300 802 [10]) may be adopted to
achieve maximum network interoperability. Other simpler or optimized solutions (e.g. to guarantee minimum signalling
delay) may be adopted to directly interface with the aforementioned DVB interactive systems.

The receiver shall evaluate quality-of-reception parameters, in particular carrier to noise plus interference ratio in dB
available at the receiver, indicated as CNI. CNI format shall be:

CNI = 150 + 10 {10 Log10[C/(N + I)]} (positive integer, 9 bits, in the range 0 to 511).

In fact for DVB-S2X 10 Log10[C/(N + I)] may be in the range -15 dB to + 36,1 dB.

10 Log10[C/(N + I)] shall be evaluated with a quantized accuracy better than 0,5 dB (accuracy = mean error + 3 σ,
where σ is the standard deviation). Since modulation and coding modes for DVB-S2X are typically spaced less than
1 dB apart, a quantized precision better than 0,2 dB is recommended in order to fully exploit system capabilities. The
measurement process is assumed to be continuous. A possible method to evaluate CNI is by using symbols known
a-priori at the receiver, such as those in the SOF field of the PLFRAME Header and, when available, Start-of-Super-
Frame preamble (SOSF), Super-Frame Format Indicator (SFFI) and pilot symbols.

CNI and other optional reception quality parameters (such as for example the BER on the channel evaluated by
counting the errors corrected by the LDPC decoder, the packet error rate detected by CRC-8, the CNI distance from the
QEF threshold) may optionally be used by the receiver to identify the maximum throughput DVB-S2X transmission
mode that it may decode at QEF, indicated by MODCOD_RQ (9 bits, b8, ..., b0) where:

•

•

•

•

b0 = 0 indicates DVB-S2 modulation and coding modes. In this case, (b5, ..., b1) are coded according to
Table 12 in ETSI EN 302 307-1 [3] and b6 is reserved for future use;

b0 = 1 indicates DVB-S2X modulation and coding modes. In this case (b6, ..., b1) are coded according to
Tables 17a and 17b. The PLS code decimal value is derived from (1, b1, b2, …, b6, 0);

b7 indicates the presence/absence of pilots: (b7 = 0 no pilots, b7 = 1 pilots). Only pilots inserted in the
PLFRAME as specified in clause 5.5.3 of ETSI EN 302 307-1 [3] are meant here. The choice whether to insert
or not SF aligned pilots in case the SF is used, is left exclusively to the Gateway;

b8 = 1 indicates (b7, ..., b0) are valid; b8 = 0 indicates (b7, ..., b0) information is not available by the terminal.

ETSI

110

ETSI EN 302 307-2 V1.4.1 (2024-08)

As a minimum, the CNI and MODCOD_RQ parameters shall be sent to the satellite network operator Gateway every
time the protection on the DVB-S2X channel has to be changed. When no modification of the protection level is
requested, the optional message from the terminal to the Gateway shall indicate MODCOD_RQ = actual MODCOD
and pilot configuration of the frames received by the terminal. In specific applications, CNI and MODCOD_RQ fields
may be extended to an integer number of byte(s), by padding zeroes in MSB positions.

The maximum delay required for CNI and MODCOD evaluation and delivery to the Gateway via the interaction
channel shall be no more than 300 ms, but this delay should be minimized if service interruptions are to be avoided
under fast fading conditions (C/N+I variations as fast as 0,5 dB/s to 1 dB/s may occur in Ka band). Optionally the
gateway may acknowledge the reception of the message and the execution of the command by a message containing the
new adopted MODCOD, coded according to Table 12 of ETSI EN 302 307-1 [3], or to Table 17a. The allocated
protection shall be equal or more robust than that requested by the terminal.

Example Transmission Protocol (ETSI EN 302 307-1 [3], reference (11))

DVBS2X_Change_MODCOD message shall be sent from the receiving terminal to the satellite network operator
gateway, every time the protection on the DVB-S2X channel has to be changed.

DVBS2X_Change_MODCOD()  length in bits (big-endian notation)
{

CNI;
9
MODCOD_RQ;  9

}

DVBS2X_Ack_MODCOD message shall optionally be sent from the Gateway to the receiving terminal to
acknowledge the DVB-S2X protection level modification. MODCOD_ACK shall be coded according to the
MODCOD_RQ conventions.

DVBS2X_Ack_MODCOD() length in bits (big-endian notation)
{

MODCOD_ACK; 9

}

ETSI

111

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex E (normative):
Super-Framing Structure (optional)

E.1

Purpose of Super-Framing Structure

The insertion of the super-framing structure is optional and has the following targets:

•

•

•

Increased resilience to co-channel interference caused by other beams for DTH and broadband applications
due to super-frame-wide scrambling.

Support of synchronization algorithms due to the regular insertion of reference data fields, which leads to
enhanced receiver performance under severe channel conditions like VL-SNR or link interruptions.

Future proof frame design with content format signalling, which is able to accommodate/support:

-

-

Interference mitigation techniques.

Beam hopping operations.

-  Multi-format transmission.

The super-framing structure is optional. Furthermore, all super-frame formats are individually optional because the
formats may differ noticeably in structure. Thus, the following labelling and behaviour shall be taken into account:

•

•

•

"Compliant to the super-frame option" means that the super-framing structure is respected and at least one
content format is supported.

In case multiple content formats are supported, it shall be indicated whether "static selection of a content
format" or a "dynamic selection between content formats" is provided. The latter case corresponds to the
capability to process a time-multiplex of different content formats.

If a receiver detects an unsupported content format, it shall skip the actual super-frame.

E.2

Specification of Super-Frame as a Container

E.2.1  Super-Frame Structure

The super-framing concept is defined to have constant length super-frames (SF) comprising SFL symbols; for Super-
Frame Formats 0, 1, 2, 3 and 4, SFL=612,540 symbols, while for Super-Frame Formats 5, 6 and 7, SFL can be selected
by the network operator.

Each super-frame comprises, at its beginning, a Start-Of-Super-Frame preamble (SOSF) and a Super-Frame Format
Indicator (SFFI), which fill the first 720 symbols. The remaining part of the super-frame can be allocated by the
payload, i.e. PLHEADERs, XFECFRAMEs, and pilot fields.

Figure E.1: Super-frames of length SFL symbols - the super-frame format specifies
the resource allocation and content

ETSI

112

ETSI EN 302 307-2 V1.4.1 (2024-08)

According to Figure E.1, the parameters and rules are:

•

•

The super-frame length is fixed to a unique number of symbols SFL (=612,540 symbols in format 0 to format
4 specific SF Formats). However, it may vary in Formats 5, 6 and 7.

The super-frame length in symbols is independent of pilot settings or hosted content formats.

The SFFI signals the actual super-frame format. A format table as well as the format specifications are presented in
clause E.3.

For resource allocation of a content format, a format-individual "Capacity Unit" (CU) can be specified. It shall provide
a grid for mapping the content into the super-frame. Note to distinguish between a resource allocation grid (based on
CUs) and the payload structure (based on SLOTs). Nevertheless, the CU size can be the same as the SLOT size of
90 symbols.

Pilot fields and pilot structure can be specified for each individual super-frame format.
The first 720 symbols per each super-frame are fixed with the SOSF and SFFI.

The full super-frame can be scrambled, including also SOSF/SFFI, with two different scrambling sequences, see
clause E.2.4. The scramblers are reset with the first symbol of the SOSF sequence. SOSF and SFFI have to be
scrambled, whereas the applicability of scrambling the hosted super-frame content is defined in each individual super-
frame format.

After super-frame generation and scrambling, baseband shaping and quadrature modulation is performed as described in
ETSI EN 302 307-1 [3], clause 5.6.

E.2.2  Start of Super-Frame (SOSF) Field

The SOSF sequence comprises 270 symbols. The SOSF defining a binary sequence is composed of a 256 bit long
Walsh-Hadamard (WH) sequence plus padding of 14 bits. Thus, a set of 28 = 256 orthogonal WH sequences results
from the following recursive construction principle:

H

(cid:0)(cid:2)

=

Apply

(cid:2)

(cid:0)

H
H

(cid:2)

(cid:2)

H
(cid:2)
-H

(cid:2)

 starting from H1 = [1] until H256 is deduced.

The i-th row of H256 corresponds to the i-th WH sequence with i = 0, …, 255. For the sake of padding, a matrix of
size 256 × 14 is appended. This matrix is generated from H16 by deleting the first and the last column, i.e.
H14 = H16(:, 1:14), and repeat H14 vertically to get:

Putting both matrices together yields:

Hpadding = [H14; H14; …; H14].

HSOSF = [H256 Hpadding],

hosting the whole set of possible SOSF sequences hi row by row. However, the selection of i is a static choice for the
transmit signal. Different signals may feature different i-values, which are considered to be a priori knowledge for the
terminal. The default value for i is 0 if nothing else is specified. Note that not all sequences hi are fully orthogonal due
to the padding matrix properties.

Before the reference data scrambling (see clause E.2.4) is applied, the chosen sequence hi is multiplied by
(1

. The first entry of hi has to be sent first.

) / 2

j+

E.2.3  Super-Frame Format Indicator (SFFI) Field

The SFFI code is constructed from a simplex code as follows:

•

Number of information bits is 4 corresponding to the bit vector bSFFI, which refers to a super-frame format as
described in Table E.1.

ETSI

The standard simplex code has a code rate of 4/15.

113

ETSI EN 302 307-2 V1.4.1 (2024-08)

A code word results from the rule (w.r.t. operation in GF2): cSFFI= bSFFI

GSX with the generator matrix

G

(cid:3)(cid:4)

=

(cid:3)0 0 0 0 0 0 0 1
0 0 0 1 1 1 1 0
0 1 1 0 0 1 1 0
1 0 1 0 1 0 1 0

1 1 1 1 1 1 1
0 0 0 1 1 1 1
0 1 1 0 0 1 1
1 0 1 0 1 0 1

(cid:4)

.

Spreading is performed by means of bit-wise repetition of cSFFI with a repetition factor of 30, i.e. each bit of
cSFFI is transmitted 30 times, which yields the 1×450 vector xSFFI.
⋅

Overall "code rate" is RSFFI=4/(15

30) = 1/112,5.

The first entry of xSFFI is transmitted first in time.

•

•

•

•

•

Before the payload data scrambling (see clause E.2.4.) is applied to xSFFI, the spread code word is BPSK modulated by
(-2

.

(1

j+

) / 2

xSFFI + 1)

E.2.4  Two-Way Scrambling

E.2.4.0  General aspects

For scrambling, a longer scrambling sequence is employed than in standard S2 but following the same general rules as
in ETSI EN 302 307-1 [3], clause 5.5.4. Also the application of the scrambling sequence is different because a two-way
scrambling is performed.

E.2.4.1  Scrambling Sequence Generation

The scrambling code sequences shall be constructed by combining two real m-sequences (generated by means of two
generator polynomials of degree 20) into a complex sequence. The resulting sequences are the basis for a set of Gold
sequences.

Let x and y be the two m-sequences with the respective primitive polynomials (over GF2):

•

•

1+x3+x20 to construct the sequence x.

1+y2+y11+y17+y20 to construct the sequence y.

The sequence depending on the chosen scrambling code number n is denoted zn in the sequel. Furthermore, let x(i), y(i)
and zn(i) denote the i-th symbol of the sequence x, y, and zn respectively. The m-sequences x and y are constructed as:

•

Initial conditions:

-

-

x is constructed with x(0) = 1, x(1) = x(2) = ... = x(18) = x(19) = 0.

y is constructed with y(0) = y(1) =

⋯

 = y(18) = y(19) = 1.

•

Recursive definition of subsequent symbols:

-

-

x(i+20) = x(i+3) + x(i) modulo 2, i = 0,…, 220-22.

y(i+20) = y(i+17) + y(i+11) + y(i+2) + y(i) modulo 2, i = 0,…, 220-22.

ETSI

114

ETSI EN 302 307-2 V1.4.1 (2024-08)

The n-th Gold code sequence zn(i), n = 0, 1, 2,…, 220-2, is then defined as:

zn(i) = [x((i+n) modulo (220-1)) + y(i)] modulo 2, i = 0,…, 220-2.

These binary sequences are converted to integer valued sequences Rn (Rn assuming values 0, 1, 2, 3) by the following
transformation:

⋅

Rn(i)= 2

zn((i + 524 288) modulo (220-1)) + zn(i), i = 0,1,…, SFL-1.
⋅

Finally, the n-th complex scrambling code sequence CI(i) + j

CQ(i) is defined by:

Cn(i) = CI,n(i) + j

⋅

CQ,n(i) = exp(j

⋅

Rn(i)

⋅

π

/2).

E.2.4.2  Two-Way Scrambling Method

Two parallel scramblers are applied as shown in Figure E.2:

1)  Reference data scrambler with sequence CnRef(iRef) applied at least to the SOSF and potentially to SF-aligned

pilots. Alternative implementation: Table-lookup of scrambled SOSF and SF-aligned pilots.

2)

Payload data scrambler with sequence CnPay(iPay)applied at least to the SFFI.

Working principle:

•

•

•

Both scramblers are reset jointly at each super-frame start and run synchronously, i.e. iRef = iPay always holds
for the scrambling sequence indices.

At the SF start the switch, depicted in Figure E.2, is in the upper position. Then, it is switched to the lower
position at the end of SOSF until the first pilot field is encountered. At the beginning of the pilot field the
switch is moved back to upper position until the end of pilot field; the next pilot field is treated identical until
the end of the SF is reached.

In general, the scrambling code numbers nRef and nPay are different, but equal code numbers are also a valid
choice. In the latter case, both scramblers coincide to a single one.

Application:

•

•

It is mandatory to apply the reference data scrambler to the SOSF and to apply the payload data to the SFFI.
Further applicability and details are specified in each format individually.

For example, one can use the application scheme:

-

-

Reference data scrambler for SOSF and SF-aligned pilots.

Payload data scrambler for SFFI, PLH, XFECFRAMEs, and VL-SNR frames.

Figure E.2: Two-way scrambling method with two parallel scramblers and selective application

ETSI

115

ETSI EN 302 307-2 V1.4.1 (2024-08)

The selection of the scrambling code numbers nRef and nPay depends on the interference scenario faced by the system.
In a co-channel interference scenario, one may need the same scrambling sequence for reference data to exploit
orthogonality but different scrambling sequences for the payload for cross-talk resilience. The use of different
scrambling sequences allows a reduction of interference correlation between different services. For the same purpose, it
is possible to reuse a shifted version of the same sequence in different satellite beams. Furthermore n can be
unequivocally associated to each satellite operator or satellite or transponder, thus permitting identification of an
interfering signal via the scrambling "signature" detection.

Thus, the two scrambling code numbers nRef and nPay can be equal but carrier unique if only adjacent channel
interference is present. Or nPay can be unique, but nRef pair-wise equal for co-channel interfering signals.

The default values are nRef = 0 and nPay = 0. If chosen otherwise, additional side-information or signalling is required as
with the signalling of alternative scrambling sequences in ETSI EN 302 307-1 [3] and the present document. For further
information is provided by the Implementation Guidelines.

Note that as the scrambling is by a sequence of complex numbers, care should be taken by the system designer to avoid
spectrum inversion, especially in beam-hopping signals (Formats 5, 6 and 7).

E.3

Format Specifications as Super-Frame Content

E.3.0  General aspects

The SFFI specifies the content format hosted by the actual super-frame. Three different modes are possible in general:

•  Multi-format carrier:

-

Free choice from the set of available formats per super-frame. The assignment of each super-frame
content is exclusively allocable by payload of the actual content format. The result is a time-multiplex of
different super-frame formats, where the receiver can skip super-frames with not-supported or unwanted
format.

•

•

Single-format carrier:

-

All super-frames feature the same single format from the set of available formats.

Quasi-single-format carrier:

-

If (at least) two formats differ only marginally, the resource allocation can work in the same way as for
the single-format case, i.e. no format-exclusive resource allocation of consecutive super-frames by the
payload is required when switching between these specific formats.

The super-frame structure enables individual format definitions, e.g. concerning SF-aligned pilots specification, and
future formats' signalling. Table E.1 shows the specified formats with reference to according clauses for detailed
description.

ETSI

116

ETSI EN 302 307-2 V1.4.1 (2024-08)

No.

bSFFI

Name

SF-pilots

Reference clause

Table E.1: Format Specifications

0
1
2

3

4
5

6

7

0 0 0 0
0 0 0 1
0 0 1 0

0 0 1 1

0 1 0 0
0 1 0 1

0 1 1 0

0 1 1 1

8 to 15   1 0 0 0 -

1 1 1 1

DVB-S2X
DVB-S2 legacy
Bundled PLFRAMES (64 800 payload size) with
SF-Pilots
Bundled PLFRAMES (16 200 payload size) with
SF-Pilots
Flexible Format with VL-SNR PLH tracking
Periodic Beam Hopping Format with VL-SNR and
fragmentation Support
Traffic Driven Beam Hopping Format with VLSNR
support
Simplified Traffic Driven Beam Hopping Format
without VL-SNR support
Reserved

Type A, if signalled   E.3.2
Type A, if signalled   E.3.3
E.3.4
See clause E.3.4

See clause E.3.5

E.3.5

Type A, if signalled   E.3.6
E.3.7
Type A, if signalled

Type A, always on

E.3.8

Type A always on

E.3.9

E.3.10

NOTE 1:  As the PLFRAMEs of formats 0, 1, and 4, 5, 6 and 7 are always a multiple of SLOTs in length, a terminal is

enabled to perform a PLFRAME (re-) synchronization/ search on a 90-symbol-grid (= CU-grid) basis. This
grid is known as soon as super-frame synchronization has been established.

NOTE 2:  The insertion of SOSF, SFFI, and possible SF-pilots interrupts the mapping of slots to super-frame resource
allocation grid irrespective of the slot content like XFECFRAMEs or PLHEADERs or VL-SNR-frames, except
in cases specified otherwise.

E.3.1  Super-Frame-aligned Pilots (SF-Pilots)

E.3.1.0  General aspects

Super-Frame-aligned pilots are specified uniquely for each super-frame format (see Table E.1 for super-frame formats).
Super-frame-aligned pilot positions are specified in reference to the SF structure, which is in contrast to the
conventional PLFRAME related pilots.

Different design approaches for SF-Pilots are adopted according to the super-frame profile.

One design approach is to define SF-pilot patterns and positions that can fulfil the following conditions:

•

•

Regular pilot insertion, which holds also between consecutive super-frames, i.e. pilot fields will be repeated
periodically across all super-frames (a constant distance in symbols between two consecutive pilot fields
across the entire carrier).

Irrespective of the presence or absence of SF-pilots (ON or OFF), no symbol padding is required to maintain
constant super-frame size.

Considering above conditions (among other conditions for other SF profiles) a super-frame size has been carefully
selected as 612,540 symbols for formats 0 to 4. Accordingly, several possible choices of SF-pilot distances dSF and
field lengths PSF, assuming a CU length of 90 symbols, are identified as shown in Table E.2.

Table E.2: Possible configurations for SF-pilots for a CU length of 90 symbols (informative)

SF-pilot distance dSF

SF-pilot field length PSF

13 CUs = 1 170 symbols
16 CUs = 1 440 symbols
16 CUs = 1 440 symbols
18 CUs = 1 620 symbols
20 CUs = 1 800 symbols
27 CUs = 2 430 symbols
27 CUs = 2 430 symbols
NOTE:

60 symbols
36 symbols
54 symbols
40 symbols
45 symbols
30 symbols
60 symbols

The overhead was calculated for SFL = 612 540 symbols.

Overhead
(note)

4,88 %
2,44 %
3,61 %
2,41 %
2,44 %
1,22 %
2,41 %

ETSI

117

ETSI EN 302 307-2 V1.4.1 (2024-08)

Among these possible choices, a pilot field size and pilot field distance similar to DVB-S2 is selected for super-frame
profiles 0, 1, 4, 5, 6 and 7 (from Table E.1), shown in bold in Table E.2 and further elaborated in clause E.3.1.1.

It should be noted that for other super-frame profiles, such as profile 2 and 3, a different approach for pilot design is
adopted as specified in clauses E.3.4 and E.3.5.

E.3.1.1  Specification of SF-Pilots Type A

The super-frame pilots of type A follow the configuration (as per the second row of Table E.2):

•

•

•

CU size = 90 symbols,

Pilot field distance, dSF = 16 CUs = 1 440 symbols,

Pilot field size, PSF = 36 symbols.

The pilot fields of length 36 symbols are regularly inserted after each 16 CUs, counting from the start of super-frame
including the CUs for SOSF/SFFI (8 CUs in total). The regularity of the pilot grid also holds from super-frame to super-
frame in case pilots remain switched ON by format selection or format-related signalling.

The pilot fields are determined by a Walsh-Hadamard (WH) sequence of size 32 plus padding of 4 bits. Thus, a set of
25 = 32 orthogonal WH sequences results from the following recursive construction principle:
(cid:2)

(cid:0)

H

(cid:0)(cid:2)

=

Apply

H
H

(cid:2)

(cid:2)

H
(cid:2)
-H

(cid:2)

 starting from H1 = [1] until H32 is deduced.

The i-th row of H32 corresponds to the i-th WH sequence with i = 0, …, 31. For the sake of padding, a matrix of size
32 × 4 is appended. This matrix is generated from H4 by repeating H4 vertically to get:

Putting both matrices together yields:

Hpadding = [H4; H4; …; H4].

HPilotA = [H32 Hpadding],

hosting the whole set of possible pilot sequences hi row by row. However, the selection of i is a static choice for the
transmit signal. Different signals may feature different i-values, which is considered to be a priori knowledge for the
terminal. The default value for i is 0 if nothing else is specified.

Before the reference data scrambling is applied, the chosen sequence hi is multiplied by

(1

j+

) / 2

.

The first entry of hi has to be sent first.

E.3.2  Format Specification 0: DVB-S2X

E.3.2.0  General aspects

The super-frame hosts S2X PLFRAMEs as specified in the present document, including the PLFRAME scrambling but
with modified VL-SNR-frames. The SLOT content is inserted in CUs of size 90 symbols. In Figure E.3, the format
structure for resource allocation is shown for the two cases of SF-pilots ON and OFF.

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF and the SF-aligned pilots.

The payload data scrambler is applied only to the SFFI.

For PLFRAMEs and VL-SNR-frames the scrambling as specified in clause 5.5.4, is applicable.

Overhead of this format (w.r.t. SOSF, SFFI) is 0,12 % (with SF-aligned pilots OFF) or 2,56 % (with SF-aligned pilots
ON).

ETSI

118

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.3: Super-frames with resource allocation structure of format 0 or 1,
where SF-pilots are ON (upper super-frame) and OFF (lower super-frame)

E.3.2.1  Pilot structure

The regular PLFRAME-pilots as specified in ETSI EN 302 307-1 [3], clause 5.5.3 are not applicable in this format.
SF-aligned pilots of Type A (see clause E.3.1.1) are applied and can be switched ON or OFF on a per-super-frame
basis.

Thus the PLH pilot indicator bit provides the super-frame pilot signalling:

•

•

At least the last 2 complete PLHs of a super-frame indicate with their pilot bit the presence or absence of
SF-aligned pilots of Type A in the next super-frame.

All other PLHs reflect the pilot setting of the actual SF.

This rule is necessary, because the terminal needs the knowledge of pilot presence directly at super-frame start.

Note that the special VL-SNR-frame pilots (see clause E.3.2.2) are present irrespective of SF-aligned pilots are ON or
OFF. The special VL-SNR pilots cannot collide with SF-aligned pilots, since they are 90 symbols in length (= 1 CU)
and are allocated to free CUs like other payload data.

E.3.2.2  Modified VL-SNR-frame

The VL-SNR-frame specification from clause 5.5.2 is modified for transmission in format 0 regarding the pilot
structure. Special VL-SNR-frame pilots are defined by:

•

•

VL-SNR-frame pilot field size is 90 symbols.

VL-SNR-frame pilot distance is 16 SLOTs = 1 440 payload symbols.

The VL-SNR-frame pilot symbol modulation is the same as in ETSI EN 302 307-1 [3], clause 5.5.3. The pilot symbols
are scrambled with the PLFRAME scrambler. According to Figure E.4, this results in the following structures for the
two VL-SNR-frame types/sets:

•

VL-SNR set 1: medium FECFRAME size
PLH of 90 (or 180) symbols + VL-SNR-header of 900 symbols + medium FECFRAME of 30 780 symbols
(i.e. S = 342 SLOTs) + 21 special VL-SNR pilots each of 90 symbols
= total VL-SNR-frame length of 33 660 symbols (or 33 750 symbols)
= 374 (or 375) CUs are allocated by a complete VL-SNR-frame of set 1.

ETSI

119

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

VL-SNR set 2: short FECFRAME size
PLH of 90 (or 180) symbols + VL-SNR-header of 900 symbols + short FECFRAME of 14 976 symbols +
54 padding symbols (i.e. S = 167 SLOTs) + 10 special VL-SNR pilots each of 90 symbols
= total VL-SNR-frame length of 16 920 symbols (or 17 010 symbols)
= 188 (or 189) CUs are allocated by a complete VL-SNR-frame of set 2
The 54 padding symbols are appended at the end of the short FECFRAME in order to achieve a completely
filled SLOT S. However, these padding symbols are treated as VL-SNR-frame pilot symbols concerning
modulation.

Note that an SOSF+SFFI or the SF-aligned pilots can interrupt items, which span over more than one CU, such as the
VL-SNR-header.

XFECFRAME

S slots

90 symbols

Slot-1

Slot-2

Slot-S

1 or 2 slots (π/2BPSK)

16 slots (selected modulation)

90 symbols

PLHEADER

VL-SNR-Header

Slot-1

Slot-16

Pilot
block

Slot-S

900 symbols (

/2BPSK)

π

VL-SNR PLFRAME before PL Scrambling and mapping to CUs of the super-frame format

Figure E.4: Insertion of VL-SNR Headers and special VL-SNR pilots

E.3.3  Format Specification 1: DVB-S2 legacy

The super-frame hosts S2 PLFRAMEs as specified in ETSI EN 302 307-1 [3]. The SLOT content is inserted in CUs of
size 90 symbols. In Figure E.3, the format structure for resource allocation is shown for the two cases of SF-pilots ON
and OFF.

SF-aligned pilots of type A are inserted following the same rules as in clause E.3.2.1.

SF-aligned scrambling is used according to clause E.2.4.2:

•

•

The reference data scrambler is applied to the SOSF and the SF-aligned pilots.

The payload data scrambler is applied only to the SFFI.

The PLFRAME scrambling as specified in clause 5.5.4 is applicable, which includes the "set of preferred scrambling
sequences".

Overhead of this format (w.r.t. SOSF, SFFI) is 0,12 % (with SF-aligned pilots OFF) or 2,56 % (with SF-aligned pilots
ON).

E.3.4  Format Specification 2: Bundled PLFRAME (64 800

payload Size) with SF-Pilots

E.3.4.0  General aspects

This format accommodates bundled PLFRAMEs of constant length. The bundled PLFRAMEs are aligned within the
super-frame. Hence, the start of each bundled PLFRAME within a super-frame can be determined based on the super-
frame format. An overview of the super-frame structure corresponding to SF Format 2 (see Table E.1) is shown in
Figure E.5.

ETSI

120

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.5: Super-frames of format with bundled PLFRAMEs (64 800 payload size)

Resource allocation is done by means of a symbol-wise mapping into super-frame. There is no CU definition.

Overhead of this format (incl. SOSF, SFFI, PLH, Pilots) is 4,79 %.

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF and the SF-aligned pilots (pilot fields P, as shown in
Figure E.5).

The payload data scrambler is applied to the SFFI, the bundled PLFRAMEs including the PLS code,
Modulated Pilot symbols (P2 in Figure E.5) and the dummy symbols at the end of the super-frame.

E.3.4.1  Bundled PLFRAME (64 800 payload) Definition

Bundled PLFRAMEs are designed to maintain a constant PLFRAME size (measured in symbols):

•

•

•

•

•

PLFRAME payload size: 64 800 symbols.

PLHEADER: 384 symbols (6 replica of identical PLS code to allow decoding down to -10 dB SNR).

Super-frame size is set to 612 540 symbols, identical to that for all other super-frame formats.

There are 9 bundled frames per each super-frame in this format.

Each bundle contains 384 symbols of the PLHEADER, 64 800 symbols of payload, 180 known modulated
symbols (P2) from the payload constellation format, and 71 pilot fields with 36 symbols in each pilot field.
The total bundled frame length is 67 920 symbols.

•  Modulated pilots symbols are inserted after the PLH and selected from the same constellation format as the
data payload of the corresponding bundled PLFRAME. Any gateway-based payload data pre-processing
technique (pre-distortion, pre-coding) shall be applied to these pilots as well.

•

•

•

•

Pilots are always present. There are 639 fields of pilots with 36 symbols in each pilot group and repeated every
956 symbols.

The first pilot field starts at symbol 1 665 with reference to the first symbol in the super-frame.

Each super frame includes 720 symbols for SOSF and SFFI.

As shown in Figure E.5, there are 540 dummy symbols at the end of each super-frame.

ETSI

121

ETSI EN 302 307-2 V1.4.1 (2024-08)

Each bundled PLFRAME comprises multiple XFECFRAMEs with the same MODCODs and a common PLHEADER.
The overall symbol size remains constant, independent of the modulation format. Figure E.6 illustrates examples of the
structure of bundled PLFRAMEs for different modulation formats. It should be noted that the bundled PLFRAME by
definition can support other modulation format as defined in clause E.3.4.2. The actual application of each modulation
is determined according to the system scenario and the use case.

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

PLH
384 (6x64)

symbols

64APSK

64APSK

64APSK

64APSK

64APSK

64APSK

8PSK (normal FECFRAME)

8PSK (normal FECFRAME)

8PSK  (normal FECFRAME)

QPSK (normal FECFRAME)

QPSK  (normal FECFRAME)

π/2-BPSK (medium FECFRAME)

π/2-BPSK  (medium FECFRAME)

π/2-BPSK SF 2 (short FECFRAME)

π/2-BPSK SF 2 (short FECFRAME)

Bundled Frame (Long) = 64800 payload symbols

Figure E.6: Selected Examples of Bundled PLFRAMEs (64 800 payload size, pilots not shown)

The use of bundled PLFRAMEs is signalled to receivers using the format identifier field of super-frame. Table E.1
shows 2 different bundled PLFRAME formats defined in a super-frame structure.

E.3.4.2  PLHEADER Specification for Bundled PLFRAMEs (64 800 payload)

PLHEADER for bundled PLFRAME consists of 6 replica of the 64-bit PLS code defined in clause 5.5.2.4 of ETSI
EN 302 307-1 [3]. No SOF is included in the PLHEADER for the bundled PLFRAME. Thus, the PLHEADER has
384 symbols with

/2 BPSK modulation.

π

Each PLS code carries 7 signalling bits defining the MODCODs type used for the entire bundled PLFRAME. All
sub-frames within each bundle share the same MODCOD as signalled by the common PLHEADER. The PLS code
repetition (equivalent to spreading factor 6) is to allow reliable detection of the MODCODs at Very Low SNR.

When PLS signalling bits (b0, b1, b2, b3, b4, b5, b6) = (0, 0, 0, 0, 0, 0, 0) i.e. dummy PLFRAME according to the
Table 12 of clause 5.5.2.2 in ETSI EN 302 307-1 [3], bundled PLFrame length shall be 64 800 symbols. It means to
have 20 times length of a dummy PLFRAME (= 3 240 symbol length) which is composed of unmodulated symbols
(I,Q)=(

2
,

 1/

).

1/

√

√

2

For this super-frame format the MODCOD field mapping is defined as below. The signalling bits are denoted as
(b0, b1, …, b6), where b0 is the Most Significant Bit (MSB) and b6 is the Least Significant Bit (LSB).

If b0 = 0, then (b1, b2,…, b6) shall be encoded according to ETSI EN 302 307-1 [3], clause 5.5.2.3 and clause 5.5.2.2,
where b1 defines the FECFRAME size and (b2,…, b6) define the MODCODs as per clause 5.5.2.2, Table 12 of ETSI
EN 302 307-1 [3].

ETSI

122

ETSI EN 302 307-2 V1.4.1 (2024-08)

NOTE:  Although it is technically allowed to use short FECFRAMEs in this super-frame format, the actual

bundling of large number of short FECFRAMEs within one bundled frame may not have a practical
application.

If b0 = 1, then (b1, b2,…, b6) shall be encoded according to Table E.3. For VL-SNR MODCODs (namely, 65 and 108 to
112 in Table E.3), the puncturing and shortening of clause 5.5.2.6 shall not be applied. From the code performance
point of view, the MODCOD thresholds are slightly lower than those reported in Table 20b and Table 20c since there is
no code puncturing applied.

Table E.3: Super-frame Format 2 MODCOD Coding

(b0, b1, b2,…, b6)
decimal value

Canonical MODCOD name

Code Type

64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112

n/a
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Normal
Medium
Medium
Medium
Short
Short

Reserved
QPSK 2/9
QPSK 13/45
QPSK 9/20
QPSK 11/20
8APSK 5/9-L
8APSK 26/45-L
8PSK 23/36
8PSK 25/36
8PSK 13/18
16APSK 1/2-L
16APSK 8/15-L
16APSK 5/9-L
16APSK 26/45
16APSK 3/5
16APSK 3/5-L
16APSK 28/45
16APSK 23/36
16APSK 2/3-L
16APSK 25/36
16APSK 13/18
16APSK 7/9
16APSK 77/90
32APSK 2/3-L
Reserved - length 32APSK
32APSK 32/45
32APSK 11/15
32APSK 7/9
64APSK 32/45-L
64APSK 11/15
Reserved - length 64APSK
64APSK 7/9
Reserved - length 64APSK
64APSK 4/5
Reserved - length 64APSK
64APSK 5/6
128APSK 3/4
128APSK 7/9
256APSK 29/45-L
256APSK 2/3-L
256APSK 31/45-L
256APSK 32/45
256APSK 11/15-L
256APSK 3/4
BPSK 1/5
BPSK 11/45
BPSK 1/3
BPSK-S 1/5
BPSK-S 11/45

ETSI

Number of
XFECFRAME
per Bundled
Frame
n/a
2 (note 1)
2
2
2
3
3
3
3
3
4
4
4
4
4
4
4
4
4
4
4
4
4
5
5
5
5
5
6
6
6
6
6
6
6
6
7
7
8
8
8
8
8
8
2 (note 2)
2 (note 2)
2 (note 2)
2 (note 3)
2 (note 3)

123

ETSI EN 302 307-2 V1.4.1 (2024-08)

(b0, b1, b2,…, b6)
decimal value

Canonical MODCOD name

Code Type

113 to 127

Reserved

n/a

Number of
XFECFRAME
per Bundled
Frame
n/a

NOTE 1:  The shortening/puncturing as shown in Table 19a and Table19b does not apply,

nldpc = 64 800.

NOTE 2:  The shortening/puncturing as shown in Table 19a and Table19c does not apply,

nldpc = 32 400.

NOTE 3:  The shortening/puncturing as shown in Table 19a and Table19d does not apply,

nldpc = 16 200.

E.3.4.3  SF-Pilot Structure

There are two different types of pilots defined in this super-frame format. The first type is based on pilot fields of
36 symbols repeated throughout the super-frame as per the following specification:

•

•

PSF = 36 symbols;

Number of pilot fields per super-frame = 639.

The starting symbol of each pilot field, with reference to the first symbol in the super-frame, is determined as follows:

Startpilot-field(m) = 1 665 +(m-1) × 956    for   m = 1, …. , 639

Thus, the pilot fields repeat periodically within each super-frame with a repetition period of 956 symbols (as shown in
Figure E.5). It should be noted that the periodicity of pilot fields is not kept between super-frames (the distance between
the closest pilot fields of two consecutive super-frames is not 956.

The pilot positions within each super-frame are carefully selected such that pilot fields do not collide with PLHEADER
of bundled frames.

For this super-frame format the start of each PLH, with reference to the start of the super-frame, is determined as:

StartPLH(n) = 721 +(n-1) × 67 920     for   n = 1, …., 9

There are 71 pilot fields per each bundled frame (summing up to a total of 639 pilot fields). In this super-frame format,
the pilot fields are always present. There is no signalling w.r.t. pilot presence.

The pilot fields are determined by a Walsh-Hadamard (WH) sequence of size 32 plus padding of 4 bits. Thus, a set of
25 = 32 orthogonal WH sequences results from the following recursive construction principle:
(cid:2)

(cid:0)

H

(cid:0)(cid:2)

=

Apply

H
H

(cid:2)

(cid:2)

H
(cid:2)
-H

(cid:2)

 starting from H1 = [1] until H32 is deduced.

The i-th row of H32 corresponds to the i-th WH sequence with i = 0, …, 31. For the sake of padding, a matrix of size
32 × 4 is appended. This matrix is generated from H4 by repeating H4 vertically to get:

Putting both matrices together yields:

Hpadding = [H4; H4; …; H4].

HPilotA = [H32  Hpadding],

hosting the whole set of possible pilot sequences hi row by row. However, the selection of i is a static choice for the
transmit signal. Different signals may feature different i-values, which is considered to be a priori knowledge for the
terminal. The default value for i is 0 if nothing else is specified.

Before the reference data scrambling is applied, the chosen sequence hi is multiplied by

(1

j+

) / 2

.

The first entry of hi has to be sent first.

ETSI

124

ETSI EN 302 307-2 V1.4.1 (2024-08)

In addition to pilot fields described above, each bundled PLFRAME also includes 180 known symbols inserted after the
PLH, as shown in Figure E.5 as P2, with a modulation similar to the corresponding bundled PLFRAME. These symbols
are defined as follows.

For bundled frames with BPSK, QPSK and 8PSK modulations:

•

Define sequence v'=[ 1     1     1    -1    -1    -1     1    -1    -1     1    -1     1 ]

•  Multiply the sequence v' by

1(

j+

2/)

•

Repeat the sequence 15 times to obtain 180 symbols

For bundled frames with 8APSK, 16APSK, 32APSK, 64APSK, 128APSK and 256APSK modulations:

•

•

•

•

•

•

•

Denote by m' the index of the MODCOD used in the corresponding bundled PLFRAME.

Denote by M the number of constellation points for MODCOD m', M = 8, 16, 32, 64, 128 or 256.

Define L =log2 (M), L = 3, 4, 5, 6, 7, 8.

The P2 pilot field is v=[v0, v1, …, v179] where each element is a constellation point from MODCOD m'.

The mapping between labels and constellation points is provided by the mapping function vi=fmod(xi, m')
where xi is a L-bits label and vi is the corresponding constellation point as specified in clause 5.4.

Define x=fbin(z,L) the function returning the L less significant digits of the binary representation of the integer
z. For example fbin(2, 4)=(0, 0, 1, 0) and fbin(20, 4)=(0, 1, 0, 0).

The generation of the P2 pilot field v=[v0, v1, …, v 179] proceeds as follows:

For i = 0,…, 179
xi= fbin(i,L) and vi = fmod(xi, m')

E.3.5  Format Specification 3: Bundled PLFRAME
(16 200 Payload Size) with SF-Pilots

E.3.5.0  General aspects

This format accommodates bundled PLFRAMEs of constant length, which follows the same structure as in format 2,
but shorter bundled PLFRAMEs are used. The bundled PLFRAMEs are aligned within the super-frame. Hence, the start
of each bundled PLFRAME within a super-frame can be determined based on the super-frame format. An example of
the overall super-frame structure corresponding to format 3 as defined in Table E.1 is shown in Figure E.7. It should be
noted that the position of pilot or the start of bundled PLFRAME does not align with 90-symbol slots (CUs).

ETSI

125

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.7: Super-frames of format 3 with bundled PLFRAMEs (16 200 Payload Size)

Resource allocation is done by means of a symbol-wise mapping into super-frame. There is no CU definition.

Overhead of this format (incl. SOSF, SFFI, PLH, Pilots) is 4,79 %.

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF and the SF-aligned pilots (pilot fields P, as shown in
Figure E.7).

The payload data scrambler is applied to the SFFI, the bundled PLFRAMEs including the PLS code,
Modulated Pilot symbols (P2 in Figure E.7) and the dummy symbols at the end of the super-frame.

E.3.5.1  Bundled PLFRAME Definition

Short bundled PLFRAMEs are designed to maintain a constant PLFRAME size (measured in symbols):

•

•

•

•

•

PLFRAME payload size: 16 200 symbols.

PLHEADER: 256 symbols (4 replica of identical PLS code).

Super-frame size is set to 612 540 symbols, identical to that for all other super-frame formats.

There are 36 bundled frames per each super-frame in this format.

Each bundle contains 256 symbols of the PLHEADER, 16 200 symbols of payload, 96 known modulated
symbols (P2) from the payload constellation format of the corresponding PLFRAME and 9 pilot fields with
48 symbols in each pilot field. The total bundled frame length is 16 984 symbols.

•  Modulated pilots symbols are inserted after the PLH and selected from the same constellation format as the
data payload of the corresponding bundled PLFRAME. Any gateway-based payload data pre-processing
technique (pre-distortion, pre-coding) shall be applied to these pilots as well.

•

•

•

•

Pilots are always present. There are 324 fields of pilots with 48 symbols in each pilot group and repeated every
1 887 symbols.

The first pilot field starts at symbol 1 801 with reference to the first symbol in the super-frame.

Each super frame includes 720 symbols for SOSF and SFFI.

As shown in Figure E.7, there are 396 dummy symbols at the end of each super-frame.

ETSI

126

ETSI EN 302 307-2 V1.4.1 (2024-08)

Each bundled PLFRAME comprises multiple XFECFRAMEs with the same MODCODs and a common PLHEADER.
The overall symbol size remains constant, independent of the modulation format. Figure E.8 illustrates the structure of
bundled PLFRAMEs for different modulation formats, i.e.:

•

•

•

For QPSK and higher order constellations, only SHORT size FECFRAMEs are applicable.

For π/2 BPSK, only SHORT size FECFRAMEs are applicable.

Spread π/2 BPSK is not available in this format.

In this bundled PLFRAME: Only Short FECFRAMEs with modulation order up to 32APSK are considered.

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

P
L
S

PLH
256 (4x64)

symbols

32APSK (short)

32APSK (short)

32APSK (short)

32APSK (short)

32APSK (short)

16APSK
(short)

16APSK
(short)

16APSK
(short)

16APSK
(short)

8PSK (short FECFRAME)

8PSK (short FECFRAME)

8PSK  (short FECFRAME)

QPSK (short  FECFRAME)

QPSK  (short FECFRAME)

π/2-BPSK  (short FECFRAME)

Bundled Frame (short) = 16200 payload symbols

Figure E.8: Bundled PLFRAMEs of 16 200 payload size (pilots not shown)

The use of bundled PLFRAMEs is signalled to receivers using the format identifier field of super-frame. Table E.1
shows 2 different bundled PLFRAME formats defined in a super-frame structure.

E.3.5.2  PLHEADER Specification for Short Bundled PLFRAME

PLHEADER for bundled PLFRAME consists of 4 replica of the 64-bit PLS code defined in clause 5.5.2.4 of ETSI
EN 302 307-1 [3]. No SOF is included in the PLHEADER for the bundles PLFRAME. Thus, the PLHEADER has
256 symbols with

/2 BPSK modulation.

π

Each PLS code carries 7 signalling bits defining the MODCODs type used for the entire bundled PLFRAME. All
sub-frames within each bundle share the same MODCOD as signalled by the common PLHEADER. The PLS code
repetition (equivalent to spreading factor 4) is to allow reliable detection of the MODCODs at Very Low SNR.

When PLS signalling bits (b0, b1, b2, b3, b4, b5, b6) = (0, 1, 0, 0, 0, 0, 0) i.e. dummy PLFRAME according to the
Table 12 of clause 5.5.2.2 in ETSI EN 302 307-1 [3], bundled PLFrame length shall be 16 200 symbols. It means to
have 5 times length of a dummy PLFRAME (= 3 240 symbol length) which is composed of unmodulated symbols
(I,Q)=(

2
,

 1/

).

1/

√

√

2

For this super-frame format the MODCOD field mapping is defined as below. The signalling bits are denoted as
(b0, b1, …, b6), where b0 is the Most Significant Bit (MSB) and b6 is the Least Significant Bit (LSB).

ETSI

127

ETSI EN 302 307-2 V1.4.1 (2024-08)

If b0 = 0, then (b1, b2,…,b6) shall be encoded according to ETSI EN 302 307-1 [3], clause 5.5.2.3 and clause 5.5.2.2. In
this super-frame format only short FECFRAMEs are allowed. Thus, b1 = 1. The 5 LSB bits (b2,…, b6) define the
MODCODs as per clause 5.5.2.2, Table 12 in ETSI EN 302 307-1 [3].

If b0 = 1, then (b1, b2,…, b6) shall be encoded according to Table E.4. For VL-SNR MODCODs (namely, 64, 65 and 66
in Table E.4), the puncturing and shortening of clause 5.5.2.6 shall not be applied. From the code performance point of
view, the MODCOD thresholds are slightly lower than those reported in Table 20b and Table 20c since there is no code
puncturing applied.

(b0, b1, b2,…, b6)
decimal value
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84 to 127

Table E.4: Super-frame Format 3 MODCOD Coding

Canonical MODCOD Name

Code Type  Number of XFECFRAME per

Bundled Frame

BPSK 1/5
BPSK 4/15
BPSK 1/3
QPSK 11/45
QPSK 4/15
QPSK 14/45
QPSK 7/15
QPSK 8/15
QPSK 32/45
8PSK 7/15
8PSK 8/15
8PSK 26/45
8PSK 32/45
16APSK 7/15
16APSK 8/15
16APSK 26/45
16APSK 3/5
16APSK 32/45
32APSK 2/3
32APSK 32/45
Reserved

Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
Short
n/a

1 (note)
1 (note)
1 (note)
2
2
2
2
2
2
3
3
3
3
4
4
4
4
4
5
5
n/a

NOTE:

The shortening/puncturing as shown in Table 19a and Table19d does not apply, nldpc = 16 200.

E.3.5.3  SF-Pilot Structure

There are two different types of pilots defined in this super-frame format. The first type is based on pilot fields of 48
symbols repeated throughout the super-frame as per the following specification.

The super-frame pilots follow the configuration:

•

•

PSF = 48 symbols,

Number of pilot fields per super-frame = 324.

The starting symbol of each pilot field, with reference to the first symbol in the super-frame, is determined as follows:

Startpilot-field(m) = 1 801 + (m-1) × 1 887    for   m = 1, …., 324

Thus, the pilot fields repeat periodically within each super-frame with a repetition period of 1 887 symbols (as shown in
Figure E.7). It should be noted unlike Type A SF-Pilots, that the periodicity of pilot fields is not kept between
super-frames.

The pilot positions within each super-frame are carefully selected such that pilot fields do not collide with PLHEADER
of bundled frames.

For this super-frame format the start of each PLH, with reference to the start of the super-frame, is determined as:

StartPLH(n) = 721 + (n-1) × 16 984     for   n = 1, …., 36

ETSI

The SF-Pilot structure is shown in Figure E.7. The pilot fields are always present. There is no signalling w.r.t. pilot
presence.

128

ETSI EN 302 307-2 V1.4.1 (2024-08)

The pilot fields are determined by a Walsh-Hadamard (WH) sequence of size 32 plus padding of a Walsh-Hadamard
(WH) sequence of size 16. A set of 25 = 32 orthogonal WH sequences results from the following recursive construction
principle:

H

(cid:0)(cid:2)

=

Apply

(cid:2)

(cid:0)

H
H

(cid:2)

(cid:2)

H
(cid:2)
-H

(cid:2)

 starting from H1 = [1] until H32 is deduced.

The i-th row of H32 corresponds to the i-th WH sequence with i = 0, …, 31. For the sake of padding, a matrix of size
32 × 16 is appended. This matrix is generated from H16 by repeating H16 vertically to get:

Putting both matrices together yields:

Hpadding = [H16; H16].

HPilot3 = [H32  Hpadding],

hosting the whole set of possible pilot sequences hi row by row. However, the selection of i is a static choice for the
transmit signal. Different signals may feature different i-values, which is considered to be a priori knowledge for the
terminal. The default value for i is 0 if nothing else is specified.

Before the reference data scrambling is applied, the chosen sequence hi is multiplied by

(1

j+

) / 2

.

The first entry of hi has to be sent first.

In addition to pilot fields described above, each bundled PLFRAME also includes 96 known symbols inserted after the
PLH, as shown in Figure E.7 as P2, with a modulation similar to the corresponding bundled PLFRAME. These symbols
are defined as follows:

For bundled frames with BPSK, QPSK and 8PSK modulations:

•

Define sequence v'=[ 1     1     1    -1    -1    -1     1    -1    -1     1    -1     1 ]

•  Multiply the sequence v' by

1(

j+

2/)

•

Repeat the sequence 8 times to obtain 96 symbols

For bundled frames with 16APSK, and 32APSK, modulations:

•

•

•

•

•

•

•

Denote by m' the index of the MODCOD used in the corresponding bundled PLFRAME.

Denote by M the number of constellation points for MODCOD m', M = 16 or 32.

Define L =l og2 (M), L =4  or 5.

The P2 pilot field is v = [v0, v1, …, v95] where each element is a constellation point from MODCOD m'.

The mapping between labels and constellation points is provided by the mapping function vi = fmod(xi, m')
where xi is a L-bits label and vi is the corresponding constellation point as specified in clause 5.4.

Define x  =fbin(z,L) the function returning the L less significant digits of the binary representation of the
integer z. For example fbin(2,4) = (0, 0, 1, 0) and fbin(20, 4) = (0, 1, 0, 0).

The generation of the P2 pilot field v=[v0, v1, …, v95] proceeds as follows:

For i = 0,…, 95
xi = fbin(i,L) and vi = fmod(xi, m')

ETSI

E.3.6  Format Specification 4: Flexible Format with VL-SNR PLH

129

ETSI EN 302 307-2 V1.4.1 (2024-08)

tracking

E.3.6.0  General aspects

This super-frame format reuses several elements of format 0 with slight modifications and extension, which are:

•

•

•

•

•

Insertion of a Super-Frame Header (SFH) and a SFH-Trailer (ST).

No VL-SNR burst-mode operation but VL-SNR PLH tracking due to PLH spreading and pointer to the first
PLH in a super-frame.

Different PLH protection levels and PLH pointer signalled by the SFH.

Application of the two way SF-scrambler.

CU size of 90 symbols.

The resulting super-frame structure using format 4 is visualized in Figure E.9.

Figure E.9: Super-frames with resource allocation structure of format 4,
where SF-pilots are ON (upper super-frame) and OFF (lower super-frame)

The main characteristics of mapping PLFRAME into super-frames are:

•

•

•

•

Each XFECFRAME is preceded by a PLH, which forms a PLFRAME.

PLFRAMEs have no alignment with super-frames except of the CU grid.

All PLFRAMEs (including spread PLFRAMEs with the extra pilot CUs) are in length a multiple of CUs.

Individual PLFRAMEs can span over more than one super-frame.

The SFH contains a pointer to the first complete PLH occurring in the current super-frame. Thus, PLH tracking by the
terminal in VL-SNR conditions is possible.

This format introduces the following overhead:

•

•

SOSF+SFFI+SFH+ST = 0,24 % w.r.t. super-frame length.

SOSF+SFFI+SFH+ST with SF pilots = 2,67 % w.r.t. super-frame length.

ETSI

130

ETSI EN 302 307-2 V1.4.1 (2024-08)

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF, ST and the SF-aligned pilots.

The payload data scrambler is applied to the SFFI, SFH, PLH and the PLFRAMEs.

E.3.6.1  Super-Frame Header (SFH)

The SFH code is constructed as follows:

•

Number of information bits: 14; meaning and order:

1)

2)

3)

11 bit pointer to first complete PLH (counting in CUs).

1 bit SF-pilots ON/OFF: 0 = SF-pilots OFF, 1 = SF-pilots ON.

2 bits PLH protection within the current super-frame:


'00': PLH spreading = 1, BPSK modulation (standard protection)

 Highest payload spreading factor within this super-frame = 1.







'01': PLH spreading = 2, BPSK modulation (robust protection)


 Highest payload spreading factor within this super-frame = 2.

'10': PLH spreading = 5, BPSK modulation (most robust protection)

 Highest payload spreading factor within this super-frame = 5.

'11': PLH punctured, QPSK modulation (high efficiency protection)


 Only allowed for 8PSK payload MODCODS and above within this super-frame.

The applied tail-biting convolutional code of rate 1/5 with the following polynomials is equal to the one for PL
signalling in ETSI EN 302 307-1 [3], Annex M, but without puncturing, i.e. 14 input bits generate 70 output
bits:

-

-

-

-

-

G0 = [10101]

G1 = [10111]

G2 = [11011]

G3 = [11111]

G4 = [11001]

(cid:0)(cid:2)
Block-wise (meaning code-word-wise) repetition with a repetition factor of 9, which means the concatenation
(cid:0)(cid:2)(cid:3)

(cid:0)
(cid:0)(cid:2)(cid:3)

(cid:0)
(cid:0)(cid:2)(cid:3)

(cid:0)
(cid:0)(cid:2)(cid:3)

= [

…

]

(cid:0)
(cid:0)(cid:2)(cid:3)
(cid:3)

.
= 1/45

.

Overall "code rate" is

(cid:0)(cid:2)(cid:3)

•

•

•

•

SFH size is 630 BPSK symbols, which corresponds to 7 CUs.

Before the payload data scrambling is applied, the spread code word is BPSK modulated by
(1 +  1

2

)/

(cid:6)

√

 in order to meet QPSK constellation points.

(cid:4)

−2 ⋅

(cid:0)(cid:2)
(cid:0)(cid:2)(cid:3)

(cid:5)

⋅

+ 1

The maximum pointer value depends on the size of the CU and the maximum (spread) codeword length (in CUs). Thus,
for the size of the CU = 90 symbols, the pointer has to cover 11 bit. The pointer value 0 points to the first CU in the
frame, thus the start of the SOSF.

However, pointer values 0 to 15 have no meaning for pointing to the first PLH because these CUs host SOSF, SFFI, and
SFH+ST. Unless there is no meaning specified for these values like, e.g. modulator error codes, the terminal PLH
tracker should ignore it as non-valid pointing data and rely on its PLH tracking.

ETSI

E.3.6.2  SFH-Trailer (ST)

131

ETSI EN 302 307-2 V1.4.1 (2024-08)

The SFH-Trailer (ST) sequence comprises 90 symbols. The binary sequence is composed of a 64 bit long Walsh-
Hadamard (WH) sequence plus padding of 26 bits. Thus, a set of 26 = 64 orthogonal WH sequences results from the
following recursive construction principle:
H
(cid:5)
-H

(cid:8)H
H

=

(cid:4)(cid:5)

H

(cid:5)

(cid:9)

(cid:5)

(cid:5)

 starting from H1 = [1] until H64 is deduced.

Apply

The i-th row of H64 corresponds to the i-th WH sequence with i = 0, …, 63. For the sake of padding, a matrix of size
64 × 26 is appended. This matrix is generated from H32 by deleting the first three and the last three columns,
i.e. H26 = H32(:, 3:28), and repeat H26 vertically to get:

Putting both matrices together yields:

Hpadding = [H26; H26].

HST = [H64  Hpadding],

hosting the whole set of possible ST sequences hi row by row. However, the selection of i is a static choice for the
transmit signal. Different signals may feature different i-values, which is considered to be a priori knowledge for the
terminal. The default value for i is 0 if nothing else is specified. Note that not all sequences hi are fully orthogonal due
to the padding matrix properties.

Before the reference data scrambling (see clause E.2.4) is applied, the chosen sequence hi is multiplied by
(1

. The first entry of hi has to be sent first.

) / 2

j+

E.3.6.3  Physical Layer Header (PLH)

E.3.6.3.0

General aspects

The PLH is constructed from a concatenation of a SOF and a PLSCODE (20 symbols and 160 symbols for standard
protection). It is closely related to the PLH definition in ETSI EN 302 307-1 [3], Annex M but without puncturing of
the PLSCODE. Here, four protection levels of the PLH are specified, which use different modulation and spreading.

E.3.6.3.1

PLSCODE Definition

The PLSCODE is constructed in analogy to ETSI EN 302 307-1 [3], Annex M. The definition for standard protection is
as follows:

•

•

Number of information bits: 16; meaning and order:

1)

2)

8 bits MOD/COD/SPREAD/SIZE, see clause E.3.6.3.3.

8 bits for TSN according to application, see Annex M of ETSI EN 302 307-1 [3].

Tail-biting convolutional code of rate 1/5 with the following polynomials (identical to SFH), i.e. 16 input bits
generate 80 output bits:

-

-

-

-

-

G0 = [10101]

G1 = [10111]

G2 = [11011]

G3 = [11111]

G4 = [11001]

ETSI

132

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

•

(cid:0)(cid:2)
Block-wise (meaning code-word-wise) repetition with a repetition factor of 2, which means the concatenation

(cid:0)

(cid:0)

= [

(cid:6)(cid:7)(cid:3)

(cid:6)(cid:7)(cid:3)

(cid:6)(cid:7)(cid:3)

 ]

.

Overall "code rate" is 1/10, which corresponds to the standard protection like in ETSI EN 302 307-1 [3],
Annex M.
This is the basis for the on-top definition of the PLH protection levels, which specifies puncturing, modulation,
and spreading.

The PLH (SOF and PLSCODE) is scrambled with the payload data scrambler. The PLSCODE-related scrambling from
ETSI EN 302 307-1 [3], clause M.2.1 is not applicable for this format.

E.3.6.3.2

PLH Protection Levels

As signalled via the SFH, four different PLH protection levels are possible, see Table E.5, which holds for all PLHs in a
super-frame. The spreading factors refer to block-wise repetition. The modulation of the PLSCODE can be:
)/

⋅ (1 + 1

−2 ⋅

+ 1

2

√

(cid:0)(cid:2)

(cid:4)

(cid:5)

(cid:6)

•

BPSK defined by

(cid:6)(cid:7)(cid:3)

; or

•

QPSK as specified in ETSI EN 302 307-1 [3], clause 5.4.1.

The high efficiency protection requires a puncturing of the PLSCODE. The bits with the following indices are
punctured:

0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 84, 92, 100, 108, 116, 124, 132, 140, 148, 156.

The resulting overall code rate is 1/8,75 in this high efficiency mode.

Table E.5: Meaning of the PLH protection levels in terms of modulation and properties

PLH protection level

Spread

Modulation

0 0 (standard prot.)
0 1 (robust prot.)
1 0 (very robust prot.)
1 1 (high efficiency)

1
2
5
1

BPSK
BPSK
BPSK
QPSK + Punct.

Overall Code Rate

(cid:0)
(cid:0)
(cid:0)
(cid:0)

(cid:0)(cid:2)(cid:3)

(cid:0)(cid:2)(cid:3)

(cid:4)(cid:4)
,
(cid:4)(cid:5)
,
(cid:5)(cid:4)
,
(cid:5)(cid:5)
(cid:0)(cid:2)(cid:3)
,

(cid:0)(cid:2)(cid:3)

= 1/10
= 1/20
= 1/50
= 1/8,75

Num. SLOTs

2
4
10
1

The resulting four different PLH structures are visualized in Figure E.10.

Protection level 0:

2 SLOTs (BPSK)

Protection level 3:

1 SLOT

PLHEADER 0

PLHEADER 3

SOF

PLSCODE

SOF

(punctured) PLSCODE

Protection level 1:

PLHEADER 0

PLHEADER 0

BPSK

QPSK

Protection level 2:

PLHEADER 0

PLHEADER 0

PLHEADER 0

PLHEADER 0

PLHEADER 0

Figure E.10: Structure of the different PLHEADER protection levels

ETSI

133

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.6.3.3

Signalling of MOD/COD/SPREAD/SIZE

The definition of ETSI EN 302 307-1 [3], Annex M is reused, but modified as follows:

•

(u0, u1, u2, u3, u4, u5, u6, u7) = 8 bit PLS Code signalling for MOD/COD/SPREAD/SIZE signalling according
to the following cases:

If u0 = 0, the following MOD/COD/SPREAD/SIZE signalling is applicable:

•

•

•

•

If u1 = 0, the decimal values of (u2, u3, u4, u5, u6) correspond for the decimal value range 1D…28D to the
MODCODs of Table 12 of ETSI EN 302 307-1 [3].

u7 signals the SIZE (0 = normal, 1 = short, not available for FEC 9/10)
The PLS Code signalling value is derived from (u0, u1, u2, u3, u4, u5, u6, u7) with the decimal value range
2D…56D.
The size information in CUs is part of Table E.8.

For a conventional Dummy PL Frame with MODCOD = 0D the PLS Code signalling value (u0, u1, u2, u3, u4,
u5, u6, u7) = 0D.

If u1 = 1, the PLS Code signalling values for (u1, u2, u3, u4, u5, u6, u7) are defined by the MODCODs of
Table E.6 with a decimal value range 64D…79D.
u7 signals the SIZE (0 = normal or medium, 1 = short) .
The PLS Code signalling value is derived from (u0, u1, u2, u3, u4, u5, u6, u7) with the decimal value range
64D…127D.
The size information in CUs is listed in Table E.8.

Table E.6: Mod/Cod/Spread Coding

Annex-I Index

PLS code decimal value
(u0, u1, u2, u3, u4, u5, u6, u7)

0

1

2

3

4

5

6

7

8

9

10

11

12

13

14

64D
65D
66D

67D

68D

69D

70D

71D

72D
73D
74D

75D

76D

77D

78D

MOD/COD/SPREAD

QPSK, 1/5, Spreading 5

QPSK, 1/4, Spreading 5

QPSK, 1/4, Spreading 5

QPSK, 1/3, Spreading 5

QPSK, 1/3, Spreading 5

QPSK, 2/5, Spreading 5

QPSK, 2/5, Spreading 5

QPSK, 1/5, Spreading 2

QPSK, 1/4, Spreading 2

QPSK, 1/4, Spreading 2

QPSK, 1/3, Spreading 2

QPSK, 1/3, Spreading 2

QPSK, 2/5, Spreading 2

Comment
(code definition)

medium size (the present document,
Table C.8). See note 1
Not defined
normal size (ETSI EN 302 307-1 [3],
Table B.1)
short size (ETSI EN 302 307-1 [3],
Table C.1). See notes 1 and 2
normal size (ETSI EN 302 307-1 [3],
Table B.2)
short size (ETSI EN 302 307-1 [3],
Table C.2). See note 1
normal size (ETSI EN 302 307-1 [3],
Table B.3)
short size (ETSI EN 302 307-1 [3],
Table C.3)
medium size (the present document,
Table C.8). See note 1
Not defined
normal size (ETSI EN 302 307-1 [3],
Table B.1)
short size (ETSI EN 302 307-1 [3],
Table C.1), See notes 1 and 2
normal size (ETSI EN 302 307-1 [3],
Table B.2)
short size (ETSI EN 302 307-1 [3],
Table C.2). See note 1
normal size (ETSI EN 302 307-1 [3],
Table B.3)

ETSI

Annex-I Index

PLS code decimal value
(u0, u1, u2, u3, u4, u5, u6, u7)

MOD/COD/SPREAD

Comment
(code definition)

134

ETSI EN 302 307-2 V1.4.1 (2024-08)

15

79D

short size (ETSI EN 302 307-1 [3],
Table C.3). See note 3
NOTE 1:  The shortening/puncturing as shown in Table 19a, Table 19c and Table 19d does not apply.
NOTE 2:  These code rates are effectively rate 1/5, short.
NOTE 3:  Same efficiency and similar error performance as MODCOD QPSK, 1/4, short.

QPSK, 2/5, Spreading 2

If u0 = 1, there is a MOD/COD/SIZE table according to clause 5.5.2.2, Table 17a. It is applicable but with the
modifications as listed in Table E.7. Note that u7 does not signal NORMAL/SHORT. It is set constant to u7 = 0, which
leads to even PLS code decimal values, indicating that frame aligned pilots are off. u7 = 1 would lead to odd PLS codes
which according to clause 5.5.2 would indicate that frame aligned pilots are on. But frame aligned pilots are not used
here, so PLS codes with u7 = 1 are all RFU. The size information in CUs is listed in Table E.8.

Table E.7: Mod/Cod/Size Coding

PLS code decimal value
(u0, u1, u2, u3, u4, u5, u6, u7)

MOD/COD/SIZE

Comment
(code definition)

128D - 131D
132D - 248D

249D - 255D

RFU
See Table 17a,
if included there, otherwise RFU, odd
values are all RFU
RFU

(clause 5)

E.3.6.3.4  Field for TSN
Besides the original meaning of the TSN field, two values are predefined:

•

•

255: Dummy frames with deterministic content as specified in clause E.3.6.7.1.

254: Dummy frames with arbitrary (modulator specific) content but following the rules stated in
clause E.3.6.7.2.

When applied in the meaning of a TSN in wideband transmission, Annex M and the Annex M of ETSI
EN 302 307-1 [3] as well as the Implementation Guidelines contain slicing rules for the modulator to respect certain
decoding capabilities of wideband terminals.

E.3.6.3.5

SOF Sequence

The SOF sequence is part of the PLH and consists of 20 known symbols. The bit sequence:

(cid:0)
(cid:0)(cid:8)(cid:2)

  =   [1 0 0 1 1 1 0 1 0 1 0 1 0 1 1 0 0 1 0 0]

defines the first 20 symbols of the PLH, where the left most MSB is transmitted first. An alternative description of the
sequence is 0x9D564.

(cid:4)

−2 ⋅

(cid:0)
(cid:0)(cid:8)(cid:2)

(cid:5)

+ 1

⋅ (1 + 1

)/

(cid:6)

√

2

. This holds irrespective of the

BPSK modulation is applied to the SOF sequence by
modulation of the PLSCODE, which can be either BPSK or QPSK.

The SOF as part of the PLH is also scrambled with the payload data scrambler.

E.3.6.4  PLFRAME structure

The specifications of XFECFRAMEs of ETSI EN 302 307-1 [3] and the present document are applicable as follows. A
PLFRAME is constructed as shown in Figure E.11 before mapping to the CUs of a super-frame. Spreading of the
XFECFRAME:

•

•

XFECFRAME spreading is signalled via PLH.

Spreading factors 1, 2, or 5 are accomplished by frame-wise repetition of the XFECFRAME.

ETSI

•

XFECFRAMEs with SPREAD > 1 contain additional pilot SLOTs as shown in clause E.3.6.5.2.

135

ETSI EN 302 307-2 V1.4.1 (2024-08)

XFECFRAME

S slots

90 symbols

Slot-1

Slot-2

Slot-S

   2 slots (BPSK)

S slots   (selected modulation)

PLHEADER

Slot-1

Slot-2

Slot-S

SOF

PLSCODE

PLFRAME before mapping to CUs of the super-frame and scrambling

Figure E.11: Structure of a PLFRAME (without spreading and PLH protection level 0)

Table E.8 defines the resulting codeword lengths (in CUs) per combination of MOD/SPREAD and SIZE.

Table E.8: XFECFRAME lengths in CUs according to MOD, SPREAD, and SIZE

2

2

2

3

4

5

6

7

8

5
1 920
(note)

2
768
(note)

2

2

5
960
(note)

2
384
(note)

2

2

5
480
(note)

2
192
(note)

5
36

2
36

1
360

1
240

1
180

1
144

1
120

1
103

1
90

2

1
-

2

1
90

1
36

3

1
-

3

4

1
-

4

5

1
-

5

1
60

1
45

1
36

1

1

1

6

1
-

6

1
-

1

7

1
-

7

1
-

1

8

1
-

8

1
-

1

Modulation
bit/symbol
ƞ
 MOD
Spreading
CUs, Normal
XFECFRAME

Modulation
bit/symbol
ƞ
 MOD
Spreading
CUs, Medium
XFECFRAME

Modulation
bit/symbol
ƞ
 MOD
Spreading
CUs, Short
XFECFRAME

Spreading
CUs, following
PLHEADER,
Conventional
Dummy PL
Frame
NOTE:

XFECFRAMEs with SPREAD > 1 contain additional pilots SLOTs, which are included in the length
calculation.

The PLFRAMEs are scrambled with the payload data scrambler, see clause E.2.4. The PLFRAME-related scrambling
from ETSI EN 302 307-1 [3], clause 5.5.4 is not applicable for this format.

ETSI

136

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.6.5  Pilot structure

E.3.6.5.1

SF-Pilots

In case the super-frame shall consist of regular pilots, "pilots ON/OFF" within the SFH code is set to "1" = "ON".
SF-aligned pilots of Type A (see clause E.3.1.1) are applied, i.e. pilot fields of length 36 symbols are regularly inserted
after each 16 CUs, counting from the start of super-frame including the CUs for SOSF/SFFI/SFH/ST (16 CUs in total).
The regularity of the pilot grid also holds from super-frame to super-frame in case pilots remain switched ON.

E.3.6.5.2

Special VL-SNR Pilots

In case the current PLH indicates a spreading factor > 1 for the actual XFECFRAME, additional CUs are dedicated as
pilot sequences in order to achieve a robust phase estimation:

•

•

•

Special VL-SNR pilot distance: 15 payload SLOTs.

Pilot fields each of 90 symbols length.

Constant I/Q symbols with constellation point

(1 + 1

(cid:6)

√

2.

)/

As these pilot fields are multiplexed with the payload data, they are also scrambled with the payload data scrambling. In
all following figures showing possible super-frame configurations, standard SF-pilots are marked with P and the special
VL-SNR pilot fields are marked by P'. This is reflected also by the exemplary short-size PLFRAME with spreading 2 in
Figure E.12.

The extra pilot insertion is only triggered by the PLH by the usage of spreading > 1 for the actual XFECFRAME. Such
case can only occur in configurations, where the SFH signals that PLH spreading is activated by means of the PLH
protection. However, even in super-frames with super-frame pilots = OFF, the extra pilot fields will be available. A
potential use-case may be a VL-SNR CCM transmission.

Super-frame CUs

...

155

156

157 158 159 160

P

161

PLFRAME SLOTs

...

PLH

PLH

1

P

2

...

...

173

174

175

176 P

177

14

15

P‘

16

P

17

...

...

189

190

191

192 P 193

29

30

P‘

31

P

32

...

...

254

255

256

P

257 258

...

90

P‘

1

P

2

...

Spread Factor = 2, low-SNR Pilots P‘ each 15 SLOTs

Figure E.12: Exemplary short-size PLFRAME with spreading 2 and
VL-SNR pilots P' together with the super-frame-aligned pilots P

NOTE:  The last SLOT of the spread XFECFRAME is always an extra pilot field. This is due to the fact that the
size of unspread XFECFRAMEs is either 90 or 360 SLOTs for short or normal size, respectively, which
are both multiples of the extra pilot field distance of 15 SLOTs.

E.3.6.6  Spreading and Signalling Rules

Although the way of spreading is already mentioned for each element individually, a brief overview is given here since
it is the last step before mapping into the super-frame structure:

•

•

•

SFH: Frame-wise spreading/repetition by a factor 9 (static).

PLH: Frame-wise spreading/repetition by a factor 1, 2, or 5 (constant for each super-frame) as signalled via
SFH. Note that the SFH signalling is valid for the first complete PLH occurring in the current super-frame.

XFECFRAME: Frame-wise spreading/repetition by a factor 1, 2, or 5 as signalled via PLH.
E.g. the repetition of entire XFECFRAMEs with a factor of 2 means transmitting the XFECFRAME twice
consecutively. The order of SLOTs is as follows (for an exemplary spreading factor of 2 and a XFECFRAME
length of 192 CUs including the special VL-SNR pilots P'):

1, 2, 3, 4, … 15, P', 16, 17, … , 89, 90, P', 1, 2, 3, 4, … 89, 90, P'

ETSI

•

The spreading factor of the XFECFRAME (signalled by the PLH) is always less or equal to the spreading
factor of the PLH (signalled by the SFH).

137

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.6.7  Dummy PL Frame Definition

E.3.6.7.0

General aspects

In addition to the conventional dummy frame as specified in ETSI EN 302 307-1 [3], clause 5.5.1, and indicated via
MODCOD 0, further dummy frames are specified for this format.

The occurrence of this format-specific dummy PLFRAME is signalled via the PLH containing:

•

•

TSN = 255: Dummy frames with deterministic content.

TSN = 254: Dummy frames with arbitrary (modulator specific) content.

The following parameters of a dummy PLFRAME are signalled via the PLH:

•  Modulation as signalled via the MOD/COD/SPREAD/SIZE field:

-  Modulation of the dummy frame data is consistent with the payload modulation of XFECFRAMEs.

However, spreading is excluded from application for dummy frames.

COD of the dummy frame PLH shall also be considered, since different constellations for one modulation order are
possible due to, e.g. different ring radii for APSK constellations.

•

Type "A" or type "B" signalled via the SIZE (SHORT/NORMAL) indication in the PLH (see
clause E.3.6.3.3):

-

The two dummy frame types are applicable for both TSN values. In opposite to dummy frame type A,
the dummy frame of type B terminates immediately when the super-frame ends. Thus, it represents an
exception condition for the PLH tracking at the terminal. The mapping of dummy frame type to the SIZE
(SHORT/NORMAL) indication in the PLH is exploited:


SHORT size: Dummy frame type A = short XFECFRAME length, which shall be the regular
choice, if the special properties of type B are not required.



NORMAL size: Dummy frame type B = normal XFECFRAME length but terminated with end of
the actual super-frame.

NOTE:

If a dummy frame type B is transmitted in the middle of a super-frame, i.e. out of the range of terminating
with the end of the super-frame, it has the regular size of a normal XFECFRAME.

•

Length of the dummy frame is determined by the MOD/COD/SPREAD/SIZE field. The lengths in Table E.8
hold except of termination of a dummy frame type B at the end of a super-frame.

The dummy frames are scrambled like all PLFRAMEs with the payload data scrambler.

E.3.6.7.1

Dummy PL frames with deterministic content

If TSN = 255 is signalled via PLH, the dummy PL frame content consists of a sequence of bits representing one
FECFRAME and are derived from a PRBS sequence. For all modulation orders, the PRBS generator feeds its first
16 200 bits or 64 800 bits to the bit-to-symbol mapper according to the choice of a short or normal size dummy frame,
respectively.

The sequence is generated by a feed-back shift register with:

1 +

polynomial

(cid:10) 14 +

(cid:10) 15

; and

initial state 100101010000000;

•

•

See Figure E.13. This sequence, which is fed to the according bit-to-symbol mapper, has length
leads to repetitions in case of a normal size dummy frame or higher order constellations.

2(cid:9)(cid:10) = 32 767

, which

ETSI

138

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.13: Generation of PRBS sequence used as FECFRAME payload data replacement
by deterministic dummy frame content

E.3.6.7.2

Dummy PL frames with arbitrary content

If TSN = 254 is signalled via PLH, the dummy PL frame content can be an arbitrary bit or even symbol sequence
selected by the modulator. Nevertheless, the rules on short or normal sizes dummy frame still apply.

As this dummy frame content is commonly not known to the terminal, the terminal cannot exploit the content and shall
ignore these dummy frames. If applicable, the received dummy frame samples can be fed back to the modulator by a
return link not specified here.

E.3.7  Format Specification 5: Periodic Beam Hopping Format with

VL-SNR and fragmentation Support

E.3.7.0  General aspects

This format is specifically designed to support beam hopping scenarios, however this format may also be used in
continuous transmission scenarios.

A prescheduled beam hopping satellite system consists of one or several beam hopping transmission channels,
operating concurrently and periodically, to serve one or multiple cell clusters respectively. Each BHTC illuminates cells
within a cluster according to a Beam Hopping Time Plan (BHTP), A beam hopping cycle consists of an illumination
pattern of consecutive cells in each cluster. A cell illumination time is defined as Dwell Time (DT) that could vary in
time duration per cell dedicating a predefined time (dwell time) over each cell in the cluster.

Super-frame format 5 reuses several elements of format 4, including fragmentation, with slight modifications and
extension, which are:

•

•

•

Flexible setting of the Super-Frame Length SFL, in order to cope with Beam Hopping Time Plans with various
dwell times.

The adoption of bit-wise spreading (instead of block-wise spreading in Format 4) for the SFH field.

The extension or modification of SFH to 720 symbols and suppression of ST field to generate 16 protected
signalling bits.

•  MODCODs allocations are different to extract a signalling bit needed to signal end of superframe and/or end

of illumination.

In beam hopping scenarios, SF pilots are always on.

In continuous super-frame transmission scenarios, the SF pilots can be set to on or off (individually per SF).

•

•

The superframe length in this format is not fixed, but variable. It may end following any payload CU or pilot field. An
illumination dwell may be comprised of several superframes.

The length of the superframe will be determined according to the following rules:

1)  A superframe which is not the last one in a dwell time, will be of a length taken from the set: SFL = n × 1 476.
The superframe shall then be terminated by a pilot field and followed by the SOSF of the next superframe.

ETSI

139

ETSI EN 302 307-2 V1.4.1 (2024-08)

2)  The last superframe in a dwell can be of any required length (down to CU or Pilot field granularity) and shall

be terminated by a postamble as specified in clause E.3.7.8.

3)  The header of the last PLFRAME in a superframe is indicated by using a PLS bit (u7, see clause E.3.7.1).

The resulting super-frame structure using format 5 is visualized in Figures E.14. Figure E.14a shows the structure of a
superframe which is not the last one in a dwell. Figure E.14b shows the structure of the last superframe in a dwell.

Pilot field,
36 symbols

CU of 90 symbols

Scrambler
RESET

Scrambler
RESET

SOSF

SFFI

SFH

P 17 18

N

-2 N

cu

-1 N

cu

cu

P

Pilots = ON

SOSF

SFFI

SFH

17

18

N

-2 N

cu

-1 N

cu

cu

Pilots = OFF

Superframe Length = SFL symbols
Distance between 2 scrambler resets = SFL symbols

- 8 CUs or 720 symbols for SOSF + SFFI
- 8 CUs or 720 symbols for SFH
- Pilots ON/OFF can be switched each superframe
- N

 in superframe: 16n

cu

Figure E.14a: The structure of Super-frames with resource allocation structure of format 5,
which are not the last superframe in a dwell. In beam hopping scenarios, SF pilots are always on.
The Super-Frame duration is flexible (SFL symbols, granularity of 1 476 symbols)

Figure E.14b: The structure of the last super-frame in a dwell, with resource allocation structure
of format 5. In beam hopping scenarios, SF-pilots are always ON.
The Super-Frame duration is flexible (SFL symbols, granularity of 1 symbol)

To acquire the transmission burst, even with very low SNR reception, the minimal length of a dwell shall be
6 × 1 476 = (8 856) symbols plus the length of the postamble.

The maximal length of a superframe is not limited.

The main characteristics of mapping PLFRAME into super-frames, as per format 4, are:

•

•

Each XFECFRAME is preceded by a PLH, which forms a PLFRAME.

PLFRAMEs have no alignment with super-frames except of the CU grid.

ETSI

140

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

•

All PLFRAMEs (including spread PLFRAMEs with the extra pilot CUs) are in length a multiple of CUs.

Individual PLFRAMEs can span over more than one super-frame.

The SFH contains a pointer to the first complete PLH occurring in the current super-frame. Thus, PLH tracking by the
terminal in VL-SNR conditions is possible. Fragmentation of data between superframes is supported. PLH shall not be
fragmented between superframes (for example, by adding an additional segment of 16 CUs and a pilot field, or by
terminating the superframe with a postamble, as defined in clause E.3.7.8).

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF, and the SF-aligned pilots.

The payload data scrambler is applied to the SFFI, SFH, PLH and the PLFRAMEs.

E.3.7.1  Super-Frame Header (SFH)

The SFH code is constructed as follows:

•

Number of information bits: 16; meaning and order:

1)

2)

3)

4)

11 bit pointer to first complete PLH (counting in CUs).

2 bits PLI (PLH protection level index) within the current super-frame:


'00': PLH spreading = 1, BPSK modulation (standard protection)

 Highest payload spreading factor within this super-frame = 1.







'01': PLH spreading = 2, BPSK modulation (robust protection)


 Highest payload spreading factor within this super-frame = 2.

'10': PLH spreading = 5, BPSK modulation (most robust protection)

 Highest payload spreading factor within this super-frame = 5.

'11': PLH punctured, QPSK modulation (high efficiency protection)


 Only allowed for 8PSK payload MODCODS and above within this super-frame.

1 bit indicates pilots on/off.

2 bits allocated to system level signalling, free for allocation by the system implementor, Default value is
all zeros. Examples are given in the implementation guidelines.

The applied tail-biting convolutional code of rate 1/5 with the following polynomials is equal to the one for PL
signalling in ETSI EN 302 307-1 [3], Annex M, but without puncturing, i.e. 16 input bits generate 80 output
bits:

-

-

-

-

-

G0 = [10101]

G1 = [10111]

G2 = [11011]

G3 = [11111]

G4 = [11001]

Bit-wise repetition with a repetition factor of 9.
(cid:3)

Overall "code rate" is

(cid:0)(cid:2)(cid:3)

= 1/45

.

SFH size is 720 BPSK symbols, which corresponds to 8 CUs.

Before the payload data scrambling is applied, the spread code word is BPSK modulated by
(1 +  1

2

)/

(cid:6)

√

 in order to meet QPSK constellation points.

ETSI

(cid:4)

−2 ⋅

(cid:0)(cid:2)
(cid:0)(cid:2)(cid:3)

(cid:5)

⋅

+ 1

•

•

•

•

141

ETSI EN 302 307-2 V1.4.1 (2024-08)

The maximum pointer value depends on the size of the CU and the maximum (spread) codeword length (in CUs). Thus,
for the size of the CU = 90 symbols, the pointer has to cover 11 bits. The pointer value 0 points to the first CU in the
frame, thus the start of the SOSF.

However, pointer values 0 to 15 have no meaning for pointing to the first PLH because these CUs host SOSF, SFFI, and
SFH. Pointer value 0 indicates that no PLH is present in this SF but the CUs are occupied with PLFRAME data. The
values 1 to 15 should be ignored as they are pointing to non-valid data.

For implementation and latency reasons, it is recommended that a single PLFRAME should not be fragmented to more
than 2 superframes.

E.3.7.2  SFH-Trailer (ST)

ST Field is not applicable in this format.

E.3.7.3  Physical Layer Header (PLH)

E.3.7.3.0

General aspects

The PLH field shall not be fragmented over superframes or illumination dwells.

The PLH is constructed from a concatenation of a SOF and a PLSCODE (20 symbols and 160 symbols for standard
protection). It is closely related to the PLH definition in ETSI EN 302 307-1 [3], Annex M but without puncturing of
the PLSCODE. Here, four protection levels of the PLH are specified, which use different modulation and spreading.

E.3.7.3.1

PLSCODE Definition

The PLSCODE definition in this format is identical to Format 4 (see clause E.3.6.3.1).

E.3.7.3.2

PLH Protection Levels

The PLH Protection Levels in this format is identical to Format 4 (see clause E.3.6.3.2).

E.3.7.3.3

Signalling of MOD/COD/SPREAD/SIZE and TYPE

The definition of ETSI EN 302 307-1 [3], Annex M is reused, but modified as follows:

•

•

•

•

(u0, u1, u2, u3, u4, u5, u6) = 7 bits for MOD/COD/SPREAD/SIZE signalling.

(u7) = TYPE "pilot" bit used for last PLFRAME in the super-frame signalling, where applicable.
u7 = 1 signals last PLFRAME within the super-frame or last PLFRAME of the dwell time.
u7 = 0 signals PLFRAME at other positions in the super-frame.

Some non-used or reserved MODCOD from ETSI EN 302 307-1 [3], Table 12 and from the present document,
Table 17a and 17b are newly defined as new spread PLFRAMES for VL-SNR, as described in Table E.9. The
table contains the PLS code signalling values of the newly defined MODCODs with u7=0.

If u0 = 0, the decimal values of (u1, u2, u3, u4, u5,) correspond for the decimal value range 1D…28D to the
MODCODs of Table 12 of ETSI EN 302 307-1 [3].

u6 signals the SIZE (0 = normal, 1 = short) ("short" not available for 9/10 code rate)
The PLS Code signalling values derived from (u0, u1, u2, u3, u4, u5, u6, u7) for these MODCODs are with the decimal
value range 4D…113D:

•

For a conventional Dummy PL Frame with MODCOD = 0D the PLS Code signalling value (u0, u1, u2, u3, u4,
u5, u6, u7) is 0D.

ETSI

142

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

If u0 = 1, the PLS Code signalling values derived from (u0, u1, u2, u3, u4, u5, u6, u7) correspond for the decimal
value range 132D ...249D to the values of clause 5.5.2.2 Table 17a, (with u7 = 0).

Additionally, some values as included in Table E.9 are used from that decimal value range for VL-SNR MODCODS.

Annex-I
Index

0

1
2
3

4
5

6
7
8

9
10

11

12

13

14

Table E.9: Mod/Cod/Spread/Size Coding

PLS code decimal value
(u0, u1, u2, u3, u4, u5, u6, u7)
114D

MOD/COD/SPREAD

Comment
(code definition)

QPSK, 1/5, Spreading 5  medium size (the present document,

Table C.8). See note 1
Not defined

116D
118D

120D
122D

124D
126D
176D

128D

130D

252D

254D

188D

QPSK, 1/4, Spreading 5  normal size (ETSI EN 302 307-1 [3], Table B.1)
QPSK, 1/4, Spreading 5  short size (ETSI EN 302 307-1 [3], Table C.1).
See notes 1 and 3
QPSK, 1/3, Spreading 5  normal size (ETSI EN 302 307-1 [3], Table B.2)
QPSK, 1/3, Spreading 5  short size (ETSI EN 302 307-1 [3], Table C.2).

See note 1

QPSK, 2/5, Spreading 5  normal size (ETSI EN 302 307-1 [3], Table B.3)
QPSK, 2/5, Spreading 5  short size (ETSI EN 302 307-1 [3], Table C.3)
QPSK, 1/5, Spreading 2  medium size (the present document,

Table C.8). See notes 1 and 2
Not defined

QPSK, 1/4, Spreading 2  normal size (ETSI EN 302 307-1 [3], Table B.1).

See note 2

QPSK, 1/4, Spreading 2  short size, (ETSI EN 302 307-1 [3], Table C.1).
See notes 1, 2 and 3
QPSK, 1/3, Spreading 2  normal size (ETSI EN 302 307-1 [3], Table B.2).

See notes 2 and 3
QPSK, 1/3, Spreading 2  short size (ETSI EN 302 307-1 [3], Table C.2).
See notes 1 and 2

QPSK, 2/5, Spreading 2  normal size (ETSI EN 302 307-1 [3], Table B.3).

See note 2

NOTE 1:  The shortening/puncturing as shown in Table 19a, Table 19c and Table 19d does not apply.
NOTE 2:  Table 17b does not apply.
NOTE 3:  These code rates are effectively rate 1/5, short.

The bit u7 = 1 signals the last PLFRAME of the superframe, allowing then the receiver to prepare for superframe
ending or illumination dwell time ending.

At the end of illumination, a postamble follows, even before the end of this last frame due to possible fragmentation.

The size information in CUs is listed in Table E.8.

E.3.7.3.4

Field for TSN

The Field for TSN is identical to Format 4 (see clause E.3.6.3.4).

E.3.7.3.5

SOF Sequence

The SOF Sequence is identical to that defined in Format 4 (see clause E.3.6.3.5).

E.3.7.4  PLFRAME structure

The PLFRAME structure in this format is identical to Format 4 (see clause E.3.6.4).

The resulting codewords lengths (in CUs) per combination of MOD/SPRAD/SIZE is given in Table E.8.

ETSI

143

ETSI EN 302 307-2 V1.4.1 (2024-08)

However, it should be noted that, unlike Format 4, the Postamble (see clause E.3.7.7) can be inserted at any CU
boundary towards the end of the last SF of a dwell, whereby the PLFRAME can be fragmented at any CU boundary. A
PLH of the PLFRAME before the Postamble shall not be fragmented (it can be inserted at the beginning of the first SF
at the next dwell, instead the Postamble shall start at an earlier CU boundary and be as usual truncated with the end of
the SF).

E.3.7.5  Pilot structure

E.3.7.5.1

SF-Pilots

The super-frame shall always consist of regular pilots. SF-aligned pilots of Type A (see clause E.3.1.1) are applied,
i.e. pilot fields of length 36 symbols are regularly inserted after each 16 CUs, counting from the start of super-frame
including the CUs for SOSF/SFFI/SFH (16 CUs in total). For dwells that comprise of several superframes, the length of
all the superframes in the dwell but the last one, shall be taken from the set SFL = n × 1 476, in order to maintain the
regularity of the pilot grid.

If this format is used in applications with continuous super-frame operation, then SF-Pilots can also be set to off
(individually per SF). The superframe length can in this case be selected from the set SFL = n × 1 476, only with n as a
multiple of 5.

E.3.7.5.2

Special VL-SNR Pilots

The Special VL-SNR Pilots definition in this format is identical to Format 4 (see clause E.3.6.5.2).

E.3.7.6  Spreading and Signalling Rules

The Spreading and Signalling Rules in this format are identical to those of format 4 (see clause E.3.6.6), with the
exception that SFH spreading is bit-wise spreading.

E.3.7.7  Dummy PL Frame Definition

The Dummy PL Frame Definition is identical to those in Format 4 (see clause E.3.6.7).

Conventional Dummy PL Frames with MODCOD = 0 (according to ETSI EN 302 307-1 [3], Table 12) shall be
signalled with a PLS Code decimal derived from (u0, u1, u2, u3, u4, u5, u6, u7) = 0D.

E.3.7.8  Postamble Definition

The postamble, following the last superframe in a dwell, is constituted of:

≤

a specific sequence of symbols (pk), 0

 k < L , where:

L = 90, 180, 360 or 900 depending on PLI

√

(cid:6)
)/

pk = (1-2bk)(1+

2 where bk is the k-th bit of the following 900 bits sequence B.

B = [0 1 1 0 1 1 0 1 1 1 1 0 0 1 1 1 0 1 0 0 1 0 1 0 1 1 1 1 0 1 1 0 0 0 0 1 1 1 1 1 0 1 0 0 0 0 0 1 1 0 0 0 0 0 1 0 0 1 1 0 1 0 1 0 0 1 0 1 1 0 1 0 0 0 0 1

       0 1 1 1 0 1 0 1 1 1 0 0 0 0 0 1 1 1 1 0 1 1 1 1 0 1 0 1 0 1 1 1 0 1 1 0 1 0 1 0 1 0 0 0 0 0 0 0 0 1 1 0 1 1 0 1 0 0 1 1 1 1 0 1 0 0 1 1 0 0 0 0 1 0 0 1

       0 1 1 1 0 0 1 1 0 0 0 1 0 0 1 0 0 0 1 1 1 1 0 0 0 1 0 1 1 1 1 0 0 0 0 1 1 0 0 1 1 0 0 1 0 0 1 0 0 1 0 1 0 0 0 1 0 1 1 0 0 0 1 1 0 0 1 0 1 0 0 1 0 0 0 0

       0 1 0 1 0 1 1 0 0 0 0 0 0 1 0 0 0 0 0 0 1 1 1 0 1 1 0 0 1 1 1 0 0 1 0 0 1 1 1 1 1 0 0 1 0 1 1 0 0 1 0 1 1 1 1 1 1 0 1 0 1 1 0 1 0 1 1 0 0 1 1 0 1 0 0 1

       0 0 1 1 0 0 1 1 1 1 1 1 1 1 1 0 1 1 0 1 1 0 0 0 1 0 1 0 0 1 1 1 0 1 1 1 1 1 0 0 0 1 1 0 1 0 0 0 1 0 0 0 0 1 1 1 0 0 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 1 1 1

       1 0 1 1 1 0 1 1 1 0 0 0 1 1 1 0 0 1 1 1 1 0 0 1 0 0 0 0 1 0 0 0 1 1 0 0 0 1 1 1 1 1 1 0 0 1 1 0 1 1 1 1 1 1 1 0 0 0 0 0 0 0 1 0 1 1 0 1 1 1 0 1 0 0 0 1

       1 1 0 1 0 1 0 0 0 1 1 0 1 1 1 0 0 1 0 1 0 1 0 0 1 1 0 1 1 0 0 1 0 0 0 1 0 0 1 1 1 0 0 0 1 0 0 0 1 0 1 0 1 0 1 1 0 1 0 1 1 1 0 1 0 1 1 1 1 0 0 1 0 0 1 0

       1 1 1 0 0 1 1 1 0 0 0 0 0 0 1 1 1 0 1 1 1 0 1 0 0 1 1 1 1 0 1 0 1 0 0 1 0 1 0 0 0 0 0 0 1 0 1 0 1 0 1 0 1 1 1 1 1 0 1 0 1 1 0 1 0 0 0 0 0 1 1 0 1 1 1 0

       1 1 0 1 1 0 1 0 1 1 0 0 0 0 0 1 0 1 1 1 0 1 1 1 1 1 0 0 0 1 1 1 1 0 0 1 1 0 1 0 0 1 1 0 1 0 1 1 1 0 0 0 1 1 0 1 0 0 0 1 0 1 1 1 1 1 1 1 0 1 0 0 1 0 1 1

ETSI

       0 0 0 1 0 1 0 0 1 1 0 0 0 1 1 0 0 0 0 0 0 0 1 1 0 0 1 1 0 0 1 0 1 0 1 1 0 0 1 0 0 1 1 1 1 1 1 0 1 1 0 1 0 0 1 0 0 1 0 0 1 1 0 1 1 1 1 1 1 0 0 1 0 1 1 0

       1 0 1 0 0 0 0 1 0 1 0 0 0 1 0 0 1 1 1 0 1 1 0 0 1 0 1 1 1 1 0 1 1 0 0 0 0 1 1 0 1 0 1 0 1 0 0 1 1 1 0 0 1 0 0 0 0 1 1 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0

       1 0 0 0 1 0 0 0 1 1 0 0 1 0 0 0 1 1 1 0 1 0 1 0 1 1 0 1 1 0 0 0 1 1 1 0 0 0 1 0 0 1 0 1 0 1 0 0 0 1 1 0 1 1 0 0 1 1 1 1 1 0 0 1]

144

ETSI EN 302 307-2 V1.4.1 (2024-08)

This sequence B can be obtained by concatenating a first PN maximal sequence of 511 bits with polynomial
1+x2+x7+x8+x9 and seed 0 1 1 0 1 1 0 1 1 concatenated with the first 389 bits of a PN maximal sequence with
polynomial 1+x5+x9 and seed 1 0 1 0 1 1 1 0 1.

(cid:6)

√

(1 + 1

)/

2

•

A sequence of symbols defined as
implementor to accommodate for hop switching time, synchronization uncertainty and other considerations.

, the number of which is to be determined by the system

If the post-amble is to be inserted at the same time as a pilot is scheduled to be inserted then the pilot shall be inserted in
priority, the post-amble will start after the final pilots sequence. If a pilot is not scheduled then a post-amble shall start
immediately. Once a post-amble has been started the pilots are suppressed, i.e. pilots are never inserted inside a post-
amble.

The postamble protection level shall be the highest protection level in the cell. SF pilot fields occurring between the
CUs carrying the postamble shall be suppressed.

The postamble will be scrambled by the payload scrambler.

The pointer in the SFH header shall indicate the value 0, if there is no payload PLH before the postamble.

If this format is used in applications with continuous super-frame operation, postambles will not be inserted.

E.3.8  Format Specification 6: Traffic Driven Beam Hopping

Format with VL-SNR Support

E.3.8.0  General aspects

A traffic driven strategy whereby packets are transmitted as soon as they arrive into the system or into the modulator.
Thus, the actual dwell time and destination of a particular transmission is random and depends on the actual traffic to be
transmitted, rather than on a pre-scheduled plan, thus reducing queuing delay and adapting better to the actual traffic.

Super-frame format 6 reuses several elements of format 5 with slight modifications which are:

•

•

The modification of SFH to a composite 720 symbols carrying 2 protected bits.

There is no fragmentation of PLFRAMES between superframes.

The superframe length in this format is not fixed, but variable. The last PLFRAME within the last superframe in the
dwell is signalled by the bit u7 in its PLH, and is terminated by a postamble.

The resulting super-frame structure using format 6 is visualized in Figures E.15. Figure E.15a shows the structure of a
superframe which is not the last one in a dwell. Figure E.15b shows the structure of the last superframe in a dwell.

ETSI

145

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.15a: The structure of Super-frames with resource allocation structure of
format 6 which are not the last superframe in a dwell. SF-pilots are always ON. The Super-Frame
duration is flexible (SFL symbols, granularity of 90, or 36 symbols)

Figure E.15b: The structure of the last super-frame in a dwell, with resource allocation structure
of format 6. SF-pilots are always ON.
The Super-Frame duration is flexible (SFL symbols, granularity of 1 symbol)

To acquire the transmission burst, even with very low SNR reception, the minimal length of the dwell shall be at least
6 × 1 476 (8 856) symbols plus the length of the postamble. However, a shorter dwell may be used, according to a
requirement of a specific implementation (e.g. a "keep alive" case, to keep the receiver on track with the transmitter
without payload data to send).

The maximal length of a superframe is not limited.

The main characteristics of mapping PLFRAME into super-frames are:

•

•

•

Each XFECFRAME is preceded by a PLH, which forms a PLFRAME.

The first PLFRAME of the superframe is aligned with super-frame header.

All PLFRAMEs (including spread PLFRAMEs with the extra pilot CUs) are in length a multiple of CUs.

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF, and the SF-aligned pilots.

The payload data scrambler is applied to the SFFI, SFH, PLH and the PLFRAMEs.

ETSI

146

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.8.1  Super-Frame Header (SFH)

The SFH field shall be comprised of:

•

•

Extended Header Field (EHF): 504 fixed symbols.

Protection Level Indication field (PLI): 216 symbols, which would signal the PLH protection level.

The EHF field will be defined as ith row of the matrix [H252, -H252] where H252=H256(:,3:254) and H256 is as defined in
clause E.2.2 for the SOSF. The row and column count start from zero, namely H252 is derived from H256 by removing
the first 3 columns and the last column of H256.

The same row number shall be selected for the SOSF and EHF fields.

Before the payload data scrambling is applied, the chosen sequence hi is multiplied by
hi has to be sent first.

The PLH protection level, as defined in E.3.7.1, will be signalled by the sequence

⋅

'00' - A sequence of 216 "0" bits.

'01' - A sequence of 72 "0" bits followed by 144 "1" bits.

'10' - A sequence of 144 "1" bits followed by 72 "0" bits.

(1

j+

) / 2

. The first entry of

(cid:0)(cid:2)

(cid:6)(cid:7)(cid:11) as follows:

'11' - A sequence of 72 "1" bits, followed by 72 "0" bits and concluded with 72 "1" bits.

Before the payload data scrambling is applied, the PLI sequence is BPSK modulated by
in order to meet QPSK constellation points, and is transmitted following the EHF symbol sequence.

(cid:4)

−2 ⋅

(cid:0)(cid:2)

(cid:5)

⋅ (1 + 1

√

(cid:6)

)/

2

+ 1

(cid:6)(cid:7)(cid:11)

E.3.8.2  Physical Layer Trailer (ST)

The ST Field is not applicable in this format.

E.3.8.3  Physical Layer Header (PLH)

Identical to Format 5, see clause E.3.7.3.

A u7 = 1 indication in the last frame indicates the end of a superframe. (SOSF correlation runs in parallel with the PLH
decoder.)

E.3.8.4  PLFRAME structure

Identical to Format 5, see clause E.3.7.4.

E.3.8.5  Pilot structure

E.3.8.5.1

SF-Pilots

The super-frame shall always consist of regular pilots. SF-aligned pilots of Type A (see clause E.3.1.1) are applied,
i.e. pilot fields of length 36 symbols are regularly inserted after each 16 CUs, counting from the start of super-frame
including the CUs for SOSF/SFFI/SFH/ (16 CUs in total). The regularity of the pilot grid will not be necessarily
maintained.

E.3.8.5.2

Special VL-SNR Pilots

The special VL-SNR Pilots definition in this format is identical to Format 4 (see clause E.3.6.5.2).

ETSI

147

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.3.8.6  Spreading and Signalling Rules

Identical to Format 4, see clause E.3.6.6. SFH spreading does not apply.

E.3.8.7  Dummy PL Frame Definition

Identical to Format 5, see clause E.3.7.7.

E.3.8.8  Postamble Definition

Identical to Format 5, see clause E.3.7.8.

E.3.9  Format Specification 7: Simplified Traffic Driven Beam

Hopping Format without VL-SNR Support

E.3.9.0  General aspects

Format 6, clause E.3.8, covers beam-hopping operation of traffic driven modes, and at wide ranges of signal to noise
ratios from VLSNR and above. In some deployments the overhead required by VLSNR operation may be alleviated.
Format 7 is aimed for traffic driven applications in which the operating signal to noise ratio is above -3 dB, where no
PLFRAME fragmentation is required.

This super-frame format reuses several elements of format 6 with slight modifications and extension, which are:

•

•

•

No SFH and no ST.

No VL-SNR burst-mode operation.

Fixed PLH protection level.

As in format 6, the superframe length in this format is not fixed, but variable. The superframe length will be determined
according to the rules specified for format 6 (clause E.3.8).

The resulting super-frame structure using format 7 is visualized in Figures E.16. Figure E.16a shows the structure of a
superframe which is not the last one in a dwell. Figure E.16b shows the structure of the last superframe in a dwell.

Figure E.16a: The structure of Super-frames with resource allocation structure of format 7,
which are not the last superframe in a dwell. SF-pilots are always ON.
The Super-Frame duration is flexible (SFL symbols, granularity of 90 or 36 symbols)

ETSI

148

ETSI EN 302 307-2 V1.4.1 (2024-08)

Figure E.16b: The structure of the last super-frame in a dwell, with resource allocation structure
of format 7. SF-pilots are always ON. the Super-Frame duration
is flexible (SFL symbols, granularity of a 90 or 36 symbols)

To acquire the transmission burst without the EHF field, the minimal length of the dwell shall be at least 6 × 1 476
(8 856) symbols plus the length of the postamble. However, a shorter dwell may be used, according to a requirement of
a specific implementation (e.g. a "keep alive" case, to keep the receiver on track with the transmitter without payload
data to send).

The maximal length of a superframe is not limited.

The main characteristics of mapping PLFRAME into super-frames are:

•

•

•

Each XFECFRAME is preceded by a PLH, which forms a PLFRAME.

First PLFRAME is aligned with super-frame.

All PLFRAMEs are in length a multiple of CUs.

SF-aligned scrambling is used according to clause E.2.4:

•

•

The reference data scrambler is applied to the SOSF and the SF-aligned pilots.

The payload data scrambler is applied to the SFFI, PLH and the PLFRAMEs.

E.3.9.1  Superframe Header (SFH)

This field is not applicable in Format 7.

E.3.9.2  SFH-Trailer (ST)

This field is not applicable in Format 7.

E.3.9.3  Physical Layer Header (PLH)

E.3.9.3.0

General aspects

The PLH is constructed from a concatenation of a SOF of 20 symbols and a PLSCODE. It is closely related to the PLH
definition in ETSI EN 302 307-1 [3], Annex M but without puncturing of the PLSCODE and no pilot bit. Only one
protection level of the PLH is specified.

E.3.9.3.1

PLSCODE Definition

The PLSCODE definition is identical to format 5 (clause E.3.7.3.1).

ETSI

E.3.9.3.2

PLH Protection Levels

149

ETSI EN 302 307-2 V1.4.1 (2024-08)

Only a standard protection level is applicable in this format. The 0 0 (standard protection) with BPSK on 2 CUs or
SLOTs.

The modulation of the PLSCODE shall be BPSK defined by

(cid:4)

−2 ⋅

(cid:0)(cid:2)

+ 1

(cid:6)(cid:7)(cid:3)

(cid:5)

(cid:6)

√

⋅ (1 + 1

)/

2.

E.3.9.3.3

Signalling of MOD/COD/SPREAD/SIZE and TYPE

The signalling of MOD MOD/COD/SPREAD/SIZE and TYPE is identical to Format 5, however, in this format, only
SPREAD=1 is applicable.

E.3.9.3.4

Field for TSN

Identical to Format 4, see clause E.3.6.3.4.

E.3.9.3.5

SOF Sequence

Identical to Format 4, see clause E.3.6.3.5.

E.3.9.4  PLFRAME structure

Identical to Format 6 for spread = 1.

E.3.9.5  SF-Pilot structure

E.3.9.5.1

SF-Pilots

Identical to Format 5, clause E.3.7.5.1.

E.3.9.5.2

Special VL-SNR Pilots

Not Applicable.

E.3.9.6  Spreading and Signalling Rules

No Spreading is applicable in this format.

E.3.9.7  Dummy PL Frame Definition

Identical to Format 5, see clause E.3.7.7.

E.3.9.8  Postamble Definition

Identical to Format 5, see clause E.3.7.8.

E.3.10  Format Specifications 8 - 15: Reserved

The formats 8 - 15 are reserved for future use.

ETSI

150

ETSI EN 302 307-2 V1.4.1 (2024-08)

E.4

Signalling of additional reception quality parameters
via return channel (normative for Interference
Management at the Gateway)

In case interference management techniques at the gateway such as for instance pre-coding are also implemented, the
present clause is also normative and the receiver shall signal the channel estimates of the nearest interfering beams (up
to a maximum of 31 beams) via an available return channel, according to the various DVB interactive systems listed in
the clause D.5. Moreover, the receiver shall also signal the carrier to noise ratio of the useful beam, i.e. the one in which
it is located, see clause D.5.

The receiver shall estimate and report the channel transfer functions, which under the assumption of non-frequency
selective channels results in a set of complex-valued coefficients hi, where index j denotes the ith interfering beam. Such
coefficients shall be estimated exploiting the SF aligned pilots, defined by a set of 32 orthogonal Walsh-Hadamard
(WH) sequences specified in e.g. clause E.3.1.1 or E.3.4.3. The knowledge of these sequences Ci allows the receiver to
discriminate the signals coming from the 31 nearest interfering beams. The channel coefficients hi can thus be estimated
as follows, assuming ideal receiver conditions (perfect lock and coherence integration):

ˆ
=
h Ae
i
i

ϕ
j
i

=

1
P N
SF

SF

N P

p

=
1

k

=
1

j

p

( )
( )
*
j C j
i

⋅

p
x
k

p

kx is the portion of the received signal corresponding to the kth block of PSF transmitted pilots within the SF

where
and Np is the number of consecutive pilot blocks over which the estimate is averaged (its value is implementation
dependant).

The measurement and estimation process is assumed to be continuous, to be reported on the return channels through a
signalling table only when significant changes are detected. The maximum delay required for estimation and delivery to
the Gateway via the interaction channel shall be no more than 500 ms, but this delay should be minimized to maximize
capacity gain. A value not exceeding 300 ms is thus recommended.

The content of a signalling table shall remain valid until a new table is received. Its content shall completely supersede
that of the previous table, e.g. in case the newer table contains a smaller number of coefficients, all old coefficients shall
be deleted upon reception of the newer table.

Table E.10: Example Signalling Table Section based on ETSI EN 300 468 [5]

Syntax

No. of bits

Reserved
(see note)

Information

Information
Mnemonic

2

receiver_channel_estimations() {
    receiver_beam_id
    receiver_beam_whs
    receiver_cn
    beam_loop_count
    for(i=0;i< beam_loop_count;i++) {
        interfering_beam_whs
        coeff_amplitude
        coeff_phase
    }
}
NOTE:  Reserved bits are of type bslbf and shall precede the information bits on the same line.

uimsbf
uimsbf
uimbsf
uimsbf

uimsbf
uimsbf
uimsbf

5
10
10

9
5
9
5

2

3

4

•

•

receiver_beam_id: this field identifies the useful beam number of the satellite carrying the forward link. If this
field is set to 511, it means this information is not available at the receiver.

receiver_beam_whs: an integer index indicating the WH sequence used for the SF aligned pilots in the useful
beam, i.e. the one in which the receiver is located.

ETSI

151

ETSI EN 302 307-2 V1.4.1 (2024-08)

•

•

•

•

receiver_cn: an integer indicating the estimated carrier to noise ratio of the useful beam:

receiver_cn = 10 × C/N [dB] + 150

where C/N [dB] is supposed to vary between -15 dB and 36,1 dB in steps of 0,1 dB.

beam_loop_count: an integer representing the number of complex-valued channel coefficients the receiver is
signalling back to the satellite gateway. Typically this is lower than 31 in practical cases.

interfering_beam_whs: an integer index indicating the WH sequence used for the SF aligned pilots in the
interfering beam the coefficient is referring to. The loop shall never contain a value equal to
receiver_beam_whs.

coeff_amplitude: the amplitude of the channel coefficient normalized with respect to the amplitude of the
channel coefficient in the useful beam.

coeff_amplitude = -10 × (A(interfering_beam_whs) [dB] - A(receiver_beam_whs) [dB])

where A(interfering_beam_whs) [dB] - A(receiver_beam_whs) [dB] is supposed to vary between 0 and -102,3 dB in
steps of 0,1 dB.

•

coeff_phase: the phase difference between the channel coefficient of the interfering beam and that of useful
one:

coeff_phase = 128/45 × (

φ

(interfering_beam_whs) [deg] -

φ

(receiver_beam_whs) [deg]) + 512

φ

(interfering_beam_whs) [deg] -

where
steps of 0,3515625°.

φ

(receiver_beam_whs) [deg] is supposed to vary between -180° and 180° in

NOTE:  The addition of a CRC or similar means to preserve information integrity depends on the specific return

link choice and of the corresponding method to transport signalling information.

ETSI

152

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex F:
Void

ETSI

153

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex G:
Void

ETSI

154

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex H (informative):
Examples of possible use of the System

H.0  General aspects

See ETSI EN 302 307-1 [3], Annex H.

H.1  Void

H.2  Void

H.3  Void

H.4  Void

H.5  Void

H.6  Void

H.7  Satellite transponder models for simulations

See ETSI EN 302 307-1 [3], clause H.7.

In addition, Figure H.1 gives the linearized TWTA AM/AM and AM/PM characteristics, to be used to test the end-to-
end performance for transponder bandwidths both in Ku and Ka bands.

ETSI

155

ETSI EN 302 307-2 V1.4.1 (2024-08)

AM/AM

-20 -18-16 -14 -12 -10 -8 -6 -4 -2 0 2 4

IBO (dB)

Phase vs Drive (deg)

)
B
d
(

O
B
O

0
-2
-4
-6
-8
-10
-12
-14
-16
-18
-20

0

-2

-4

-6

-8

g)
e
 (d
ase
h
P

-10

-12

-20 -18 -16 -14 -12 -10 -8 -6 -4 -2

IBO(dB)

0

2

Figure H.1: Linearized TWTA Amplitude and Phase response model

In addition, Figure H.2 gives the Hard limiter Model used to derive simulation results provided in Table 20a.

ETSI

156

ETSI EN 302 307-2 V1.4.1 (2024-08)

Pout

CSAT

Pin

Figure H.2: Hard-limiter TWTA model

H.8  Phase noise masks for simulations

See ETSI EN 302 307-1 [3], clause H.8.

The following phase noise masks for consumer reception systems may be used to evaluate the carrier recovery
algorithms. The mask represents single side-band power spectral densities. The "aggregate" masks combine the phase
noise contributions of the LNB and of the relevant Tuner. Other sources of phase noise within the chain (e.g. satellite
transponder, up-link station, etc.) are usually negligible, and therefore the proposed masks may be considered as
representative of the full chain.

Table H.1: Aggregate Phase Noise masks for Simulation (in dBc/Hz)



frequency
Aggregate1 (typical)
Aggregate2 (critical)

100 Hz  1 kHz  10 kHz  100 kHz  1 MHz  > 10 MHz

-25
-25

-50
-50

-73
-73

-93
-85

-103
-103

-114
-114

Further, the following masks may be used for specific purposes.

Table H.2: Phase noise masks to be used for the DTH broadcasting services

Offset (Hz)
Typical
SSB dBc/Hz
Critical
(Symbol rates less
than
36 Mbaud)
SSB dBc/Hz

100 Hz

1 kHz

10 kHz

100 kHz

1 MHz

10 MHz

≥

 50 MHz

-25

-50

-73

-92,25

-102,49

-113,23

-115,89

-25

-50

-72,90

-84,76

-89,68

-89,68

-89,68

Table H.3: Phase noise mask proposed in TM-S20113 for professional services

Offset (Hz)
Typical
SSB dBc/Hz

10

100

1 k

10 k

100 k

1 M

10 M

≥

 50 MHz

-32,93

-61,96

-78,73

-88,73

-94,83

-105,74

-115,74

-117,74

ETSI

157

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table H.4: Phase noise masks to be used for the outbound VSAT services

Offset (Hz)
Critical mask
SSB dBc/Hz
Typical mask
SSB dBc/Hz

10 Hz

100 Hz

1 kHz

10 kHz

100 kHz

1 MHz

10 MHz

≥

 50 MHz

-27

-45

-65

-75

-89

-102

-112

-112

-32,93

-61,96

-78,73

-88,73

-94,83

-105,74

-115,74

-117,74

ETSI

158

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex I (normative):
ACM

I.1

ACM Command

(See ETSI EN 302 307-1 [3], clause I.2).

The S2X MODCODs are signalled by setting the reserved bit Acm[7] (defined in Table I.2) equal to 1. The Acm byte
will map one-to-one to the PL header bits as illustrated in Table I.1 (except for VL-SNR MODCODs used in Annex E,
Format 4, where a special PL header bit-mapping as described in clause E.3.6.3.3 is used for transmission. For b7
additional special cases apply in Annex E).

Table I.1: ACM command byte definition (Acm[0] is the least significant bit)

Bit fields

Acm[0]
Acm[1]
Acm[2]
Acm[3]
Acm[4]
Acm[5]

PL header
b5
b4
b3
b2
b1
b7

Acm[6]

b6

Description

S2 MODCOD interpretation:

•  MODCOD (as defined in ETSI EN 302 307-1 [3], Table 12)

S2X MODCOD interpretation:

•  PL header bits b5 to b1 (see Table 17a)

pilots configuration (0 = no pilots, 1 = pilots) or signalling of last frame of
an illumination (1 = last, 0 = other) in case of a beam hopping application
with Annex-E, Format 5,6,7. (See note)
S2 MODCOD interpretation:

•

FECFRAME sizes (0 = normal: 64 800 bits; 1 = short: 16 200
bits)

S2X MODCOD interpretation:

•  PL header bit b6 (see Table 17a)

Acm[7]
NOTE:

b0

Bit indicating S2 MODCOD (Acm[7]=0) or S2X MODCOD (Acm[7]=1)

For Annex E, Format 0, 1, 4, 5, 6, 7 the Acm[5] bit is ignored by devices which themselves
generate superframes, whereby the PL header bit b7 is defined internally within these
devices according to signalling requirements.

In case of S2X (non Annex E) and S2X, Annex E, Format 0, if the ACM byte points to a MODCOD belonging to the
VL-SNR range (Acm=0xA0 or Acm=0xE0) then a second ACM byte (called ACM2) is appended to signal the specific
VL-SNR MODCOD. This is illustrated in Figure I.1. A similar signalling mechanism (selecting Acm=0xA0 or
Acm=0xE0) is used also for S2X, Annex E, Format 4, 5, 6, 7 VL-SNR MODCODs.

In case of S2X, Annex E, Format 5,6,7 the VL-SNR MODCODs can alternatively be signalled using the ACM byte
only and the PLS code values listed in Table E.9, clause E.3.7.3.3.

TSHEADER

BBHEADER = 10 Bytes

PAYLOAD = DFL  bits

0xB8

ACM

ACM2

Transport Header : 3 Bytes

Figure I.1: Mode Adaptation format at the Mode Adaptation input interface
(case of S2X VL-SNR MODCOD)

In the case of Annex E, the meaning of the PLS tables apply with (u0, u1, u2, u3,.., u7) = (b0, b1, b2, b3,.., b7).

The ACM2 command byte is defined in Table I.2.

ETSI

159

ETSI EN 302 307-2 V1.4.1 (2024-08)

Table I.2: ACM2 command byte definition (acmVL-SNR[0] is the least significant bit)

Byte

Case
Acm2 (7:4)

0000
0010

Acm2(3:0)
Acm2(3:0)

0100

Acm2(3:0)

Others

Description

Index pointing to the VL-SNR MODCOD, as shown in Table 18b, clause 5.5.2.5.
Index pointing to the VL-SNR MODCOD of Annex E, Format 5, 6, 7, as shown
in Table E.9, clause E.3.7.3.3.
For clarity: The values for the Acm byte shall be Acm=0xA0 or Acm=0xE0.
Index pointing to the VL-SNR MODCOD of Annex E, Format 4, as shown in
Table E.6, clause E.3.6.3.3.
For clarity: The values for the Acm byte shall be Acm=0xA0 or Acm=0xE0.
RFU.

I.2

Dummy Synchronization Scheme (optional)

I.2.0  General aspects

The Dummy Synchronization scheme is optional and has the following objectives.

Facilitate, in specific receiver implementations, VLSNR and DVB-S2/S2X to be seamlessly mixed within the same
carrier without frame loss.

Support sparse VL-SNR signal synchronization.

The Dummy Synchronization scheme suggests that a Dummy Synchronization Frame (DSF) is inserted within the
stream of regular PL frames. This scheme is not applicable for Annex E superframes transmissions. It is intended that
the DSF be sent prior to a VLSNR frame or group of VLSNR frames and that VLSNR frames will be sent consecutively
without gaps or S2/S2X frames inserted between the VLSNR frames within the group. Once the VLSNR group has
ended PL frames can be sent in any order of MODCOD until the next VLSNR frame or group which is preceded by a
DSF. Of course, in the absence of VLSNR frames to be sent, a DSF can be sent and followed by a standard PL frame.

I.2.1  Dummy Synchronization Frame structure

I.2.1.0  General aspects

The Dummy Synchronization Frame structure is exactly the same as Dummy PL frame, without pilots, but with defined
content.

The Dummy PL Frame consists of a physical layer header (PLH*), some dummy symbols and a known correlation
structure.

The known correlation structure is in fact identical to an Annex E format 6 Superframe header.

The Dummy Synchronization Frame Length is 3 330 symbols (3 420 symbols for transmission format according to
Annex M).

ETSI

160

ETSI EN 302 307-2 V1.4.1 (2024-08)

Defined content DSF

PLH*

Scrambled known symbols

SOSF

SFFI

EHF

PLI

P

PLH

VLSNR

…

Backward
compatible
DummyPL
Header

As per standard DF

Known correlation
structure

720+504+216+36
=1476 Symbols

Next frame
e.g. VL-SNR frame in a
continuous S2X ACM
stream

Figure I.2: Dummy Synchronization Frame structure

Standard receivers will ignore the DSF treating it as if it were a standard Dummy Frame. Thus ensuring the scheme is
legacy compatible.

I.2.1.1  PLH* description

The PLHeader PLH* (90 symbols, or 180 symbols for Annex M transmission format) shall be composed of the
following fields:

•

•

SOF of 26 symbols as per clause 5.5.2.1.

PLS code of 64 symbols or 154 symbols (as per clause 5.5.2):

-  MODCOD (6 bits), Dummy Frame 0D.

-

TYPE (2 bits), with TYPE LSB always equal to 'zero' (in effect indicating no pilots) and TYPE MSB
used to discriminate between legacy dummy frames and a DSF (PLH*). It is up to the system
implementers to define within their system the assignation of this (TYPE MSB) bit i.e. in some systems
TYPE MSB equal to 'zero' would indicate PLH* and in other systems a 'one' would indicate PLH*.
Implementers shall ensure this is configurable as a system parameter.
As is customary the PLS code shall be encoded using either the Reed-Muller or when Annex M is used,
the convolutional code.

PLH* shall be modulated into

π

/2-BPSK (as per clause 5.5.2).

The scrambling and modulation of the PLH* is identical to standard PLH scrambling (see clause 5.5.2) which provides
90 or 180

/2-BPSK symbols.

π

When in Annex M format, PLH* may use the slice number to further discriminate PLH* (if necessary).

I.2.1.2  Known Symbols

√

The Dummy PL Frame shall be filled, immediately after the PLH*, with 1 764 symbols of un-modulated carriers
(I = (1/

2)), (as described in ETSI EN 302 307-1 [3], clause 5.5.1).

2), Q = (1/

√

I.2.1.3  Known correlation structure

The header structure from Annex E format 6 is re-used. The Length of the known correlation structure is of
1 476 symbols, with:

•

•

•

SOSF as per clause E.2.2 (index i=0 default value).

SFFI = "0110" as per clause E.2.3.

SFH as per clause E.3.8.1 with EHF and PLI fields:

-

PLI as per clause E.3.8.1 except that PLI value


In case of Annex M, PLI = "00". i.e. PLH of 2 CUs.

ETSI



In case of standard S2X, PLI = "11". i.e. PLH of 1 CU.

•

Pilots field as per clause E.3.1 Type A (36 symbols) (index i=0 default value).

161

ETSI EN 302 307-2 V1.4.1 (2024-08)

I.2.2  Scrambling

The entire payload part of the frame, known (dummy) symbols and known correlation structure, are scrambled as per
clause 5.5.4 with scrambler reset immediately after the PLH* (as is customary).

Scrambler
reset

Standard S2/S2X
PL scrambler as per
Para 5.5.4

Standard S2/S2X
PL scrambler as per
Para 5.5.4

PLH*

Scrambled known symbols

SOSF

SFFI

EHF

PLI

P

PLH

VLSNR

…

Figure I.3: Scrambling of the Dummy Synchronization Frame

For clarity: The Annex E reference and payload scramblers are not applied. They are replaced by the PLFrame
scrambling.

ETSI

162

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex J:
Void

ETSI

163

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex K:
For future use

ETSI

164

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex L:
For future use

ETSI

165

ETSI EN 302 307-2 V1.4.1 (2024-08)

Annex M (normative):
Transmission format for wideband satellite transponders
using time-slicing (optional)

See ETSI EN 302 307-1 [3], Annex M, where clauses M.2.3 and M.2.4 shall be replaced by the clauses below.

M.2.3

Modcod field

The first 8 bit of the information bit sequence shall be defined as follows:

(u0, u1, u2, u3,..., u7) = (b0, b1, b2, b3,..., b7)

The definition of the PLS bits (b0, b1, b2, b3,..., b7) is found in clause 5.5.2.2.

M.2.4

Type field

The type field definition (bits u6, u7) is included in the MODCOD field definition.

ETSI

166

ETSI EN 302 307-2 V1.4.1 (2024-08)

History

V1.1.1

V1.2.1

V1.3.1

V1.4.1

V1.4.1

Document history

February 2015

Publication

August 2020

Publication

July 2021

Publication

May 2024

EN Approval Procedure

AP 20240815: 2024-05-17 to 2024-08-15

August 2024

Publication

ETSI


