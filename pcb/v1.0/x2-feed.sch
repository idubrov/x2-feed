<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE eagle SYSTEM "eagle.dtd">
<eagle version="8.2.0">
<drawing>
<settings>
<setting alwaysvectorfont="no"/>
<setting verticaltext="up"/>
</settings>
<grid distance="0.1" unitdist="inch" unit="inch" style="lines" multiple="1" display="no" altdistance="0.01" altunitdist="inch" altunit="inch"/>
<layers>
<layer number="1" name="Top" color="4" fill="1" visible="no" active="no"/>
<layer number="16" name="Bottom" color="1" fill="1" visible="no" active="no"/>
<layer number="17" name="Pads" color="2" fill="1" visible="no" active="no"/>
<layer number="18" name="Vias" color="2" fill="1" visible="no" active="no"/>
<layer number="19" name="Unrouted" color="6" fill="1" visible="no" active="no"/>
<layer number="20" name="Dimension" color="15" fill="1" visible="no" active="no"/>
<layer number="21" name="tPlace" color="7" fill="1" visible="no" active="no"/>
<layer number="22" name="bPlace" color="7" fill="1" visible="no" active="no"/>
<layer number="23" name="tOrigins" color="15" fill="1" visible="no" active="no"/>
<layer number="24" name="bOrigins" color="15" fill="1" visible="no" active="no"/>
<layer number="25" name="tNames" color="7" fill="1" visible="no" active="no"/>
<layer number="26" name="bNames" color="7" fill="1" visible="no" active="no"/>
<layer number="27" name="tValues" color="7" fill="1" visible="no" active="no"/>
<layer number="28" name="bValues" color="7" fill="1" visible="no" active="no"/>
<layer number="29" name="tStop" color="7" fill="3" visible="no" active="no"/>
<layer number="30" name="bStop" color="7" fill="6" visible="no" active="no"/>
<layer number="31" name="tCream" color="7" fill="4" visible="no" active="no"/>
<layer number="32" name="bCream" color="7" fill="5" visible="no" active="no"/>
<layer number="33" name="tFinish" color="6" fill="3" visible="no" active="no"/>
<layer number="34" name="bFinish" color="6" fill="6" visible="no" active="no"/>
<layer number="35" name="tGlue" color="7" fill="4" visible="no" active="no"/>
<layer number="36" name="bGlue" color="7" fill="5" visible="no" active="no"/>
<layer number="37" name="tTest" color="7" fill="1" visible="no" active="no"/>
<layer number="38" name="bTest" color="7" fill="1" visible="no" active="no"/>
<layer number="39" name="tKeepout" color="4" fill="11" visible="no" active="no"/>
<layer number="40" name="bKeepout" color="1" fill="11" visible="no" active="no"/>
<layer number="41" name="tRestrict" color="4" fill="10" visible="no" active="no"/>
<layer number="42" name="bRestrict" color="1" fill="10" visible="no" active="no"/>
<layer number="43" name="vRestrict" color="2" fill="10" visible="no" active="no"/>
<layer number="44" name="Drills" color="7" fill="1" visible="no" active="no"/>
<layer number="45" name="Holes" color="7" fill="1" visible="no" active="no"/>
<layer number="46" name="Milling" color="3" fill="1" visible="no" active="no"/>
<layer number="47" name="Measures" color="7" fill="1" visible="no" active="no"/>
<layer number="48" name="Document" color="7" fill="1" visible="no" active="no"/>
<layer number="49" name="Reference" color="7" fill="1" visible="no" active="no"/>
<layer number="51" name="tDocu" color="14" fill="1" visible="no" active="no"/>
<layer number="52" name="bDocu" color="7" fill="1" visible="no" active="no"/>
<layer number="90" name="Modules" color="5" fill="1" visible="yes" active="yes"/>
<layer number="91" name="Nets" color="2" fill="1" visible="yes" active="yes"/>
<layer number="92" name="Busses" color="1" fill="1" visible="yes" active="yes"/>
<layer number="93" name="Pins" color="2" fill="1" visible="no" active="yes"/>
<layer number="94" name="Symbols" color="4" fill="1" visible="yes" active="yes"/>
<layer number="95" name="Names" color="7" fill="1" visible="yes" active="yes"/>
<layer number="96" name="Values" color="7" fill="1" visible="yes" active="yes"/>
<layer number="97" name="Info" color="7" fill="1" visible="yes" active="yes"/>
<layer number="98" name="Guide" color="6" fill="1" visible="yes" active="yes"/>
</layers>
<schematic xreflabel="%F%N/%S.%C%R" xrefpart="/%S.%C%R">
<libraries>
<library name="x2-feed">
<packages>
<package name="LQFP48-7X7MM">
<description>LQFP-48 package</description>
<smd name="1" x="-4.25" y="2.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="2" x="-4.25" y="2.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="3" x="-4.25" y="1.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="4" x="-4.25" y="1.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="5" x="-4.25" y="0.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="6" x="-4.25" y="0.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="7" x="-4.25" y="-0.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="8" x="-4.25" y="-0.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="9" x="-4.25" y="-1.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="10" x="-4.25" y="-1.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="11" x="-4.25" y="-2.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="12" x="-4.25" y="-2.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="13" x="-2.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="14" x="-2.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="15" x="-1.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="16" x="-1.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="17" x="-0.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="18" x="-0.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="19" x="0.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="20" x="0.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="21" x="1.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="22" x="1.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="23" x="2.25" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="24" x="2.75" y="-4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="25" x="4.25" y="-2.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="26" x="4.25" y="-2.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="27" x="4.25" y="-1.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="28" x="4.25" y="-1.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="29" x="4.25" y="-0.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="30" x="4.25" y="-0.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="31" x="4.25" y="0.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="32" x="4.25" y="0.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="33" x="4.25" y="1.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="34" x="4.25" y="1.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="35" x="4.25" y="2.25" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="36" x="4.25" y="2.75" dx="0.3" dy="1.2" layer="1" rot="R270"/>
<smd name="37" x="2.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="38" x="2.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="39" x="1.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="40" x="1.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="41" x="0.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="42" x="0.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="43" x="-0.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="44" x="-0.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="45" x="-1.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="46" x="-1.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="47" x="-2.25" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<smd name="48" x="-2.75" y="4.25" dx="1.2" dy="0.3" layer="1" rot="R270"/>
<text x="0" y="5" size="0.6096" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-5" size="0.6096" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<circle x="-2.5" y="2.5" radius="0.5" width="0.127" layer="21"/>
<wire x1="-3.1" y1="3.5" x2="-3.5" y2="3.1" width="0.127" layer="21"/>
<wire x1="-3.5" y1="3.1" x2="-3.5" y2="-3.1" width="0.127" layer="21"/>
<wire x1="-3.5" y1="-3.1" x2="-3.1" y2="-3.5" width="0.127" layer="21"/>
<wire x1="-3.1" y1="-3.5" x2="3.1" y2="-3.5" width="0.127" layer="21"/>
<wire x1="3.1" y1="-3.5" x2="3.5" y2="-3.1" width="0.127" layer="21"/>
<wire x1="3.5" y1="-3.1" x2="3.5" y2="3.1" width="0.127" layer="21"/>
<wire x1="3.5" y1="3.1" x2="3.1" y2="3.5" width="0.127" layer="21"/>
<wire x1="3.1" y1="3.5" x2="-3.1" y2="3.5" width="0.127" layer="21"/>
<rectangle x1="-4.15" y1="2.25" x2="-3.85" y2="3.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="1.75" x2="-3.85" y2="2.75" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="1.25" x2="-3.85" y2="2.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="0.75" x2="-3.85" y2="1.75" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="0.25" x2="-3.85" y2="1.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-0.25" x2="-3.85" y2="0.75" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-0.75" x2="-3.85" y2="0.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-1.25" x2="-3.85" y2="-0.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-1.75" x2="-3.85" y2="-0.75" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-2.25" x2="-3.85" y2="-1.25" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-2.75" x2="-3.85" y2="-1.75" layer="51" rot="R270"/>
<rectangle x1="-4.15" y1="-3.25" x2="-3.85" y2="-2.25" layer="51" rot="R270"/>
<rectangle x1="-3.25" y1="-4.15" x2="-2.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-2.75" y1="-4.15" x2="-1.75" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-2.25" y1="-4.15" x2="-1.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-1.75" y1="-4.15" x2="-0.75" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-1.25" y1="-4.15" x2="-0.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-0.75" y1="-4.15" x2="0.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="-0.25" y1="-4.15" x2="0.75" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="0.25" y1="-4.15" x2="1.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="0.75" y1="-4.15" x2="1.75" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="1.25" y1="-4.15" x2="2.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="1.75" y1="-4.15" x2="2.75" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="2.25" y1="-4.15" x2="3.25" y2="-3.85" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-3.25" x2="4.15" y2="-2.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-2.75" x2="4.15" y2="-1.75" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-2.25" x2="4.15" y2="-1.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-1.75" x2="4.15" y2="-0.75" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-1.25" x2="4.15" y2="-0.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-0.75" x2="4.15" y2="0.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="-0.25" x2="4.15" y2="0.75" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="0.25" x2="4.15" y2="1.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="0.75" x2="4.15" y2="1.75" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="1.25" x2="4.15" y2="2.25" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="1.75" x2="4.15" y2="2.75" layer="51" rot="R270"/>
<rectangle x1="3.85" y1="2.25" x2="4.15" y2="3.25" layer="51" rot="R270"/>
<rectangle x1="2.25" y1="3.85" x2="3.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="1.75" y1="3.85" x2="2.75" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="1.25" y1="3.85" x2="2.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="0.75" y1="3.85" x2="1.75" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="0.25" y1="3.85" x2="1.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-0.25" y1="3.85" x2="0.75" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-0.75" y1="3.85" x2="0.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-1.25" y1="3.85" x2="-0.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-1.75" y1="3.85" x2="-0.75" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-2.25" y1="3.85" x2="-1.25" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-2.75" y1="3.85" x2="-1.75" y2="4.15" layer="51" rot="R270"/>
<rectangle x1="-3.25" y1="3.85" x2="-2.25" y2="4.15" layer="51" rot="R270"/>
</package>
<package name="C1608">
<description>&lt;b&gt;CAPACITOR&lt;/b&gt;</description>
<wire x1="-1.5" y1="1" x2="1.5" y2="1" width="0.0508" layer="39"/>
<wire x1="1.5" y1="1" x2="1.5" y2="-1" width="0.0508" layer="39"/>
<wire x1="1.5" y1="-1" x2="-1.5" y2="-1" width="0.0508" layer="39"/>
<wire x1="-1.5" y1="-1" x2="-1.5" y2="1" width="0.0508" layer="39"/>
<wire x1="-0.4" y1="0.4" x2="0.4" y2="0.4" width="0.1" layer="51"/>
<wire x1="-0.4" y1="-0.4" x2="0.4" y2="-0.4" width="0.1" layer="51"/>
<smd name="1" x="-0.85" y="0" dx="1.1" dy="1" layer="1"/>
<smd name="2" x="0.85" y="0" dx="1.1" dy="1" layer="1"/>
<text x="0" y="0.75" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-0.75" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-0.8" y1="-0.45" x2="-0.4" y2="0.45" layer="51"/>
<rectangle x1="0.4" y1="-0.45" x2="0.8" y2="0.45" layer="51"/>
<rectangle x1="-0.1001" y1="-0.4001" x2="0.1001" y2="0.4001" layer="35"/>
</package>
<package name="C2012">
<description>&lt;b&gt;CAPACITOR&lt;/b&gt;</description>
<wire x1="-1.973" y1="0.983" x2="1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="0.983" x2="1.973" y2="-0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="-0.983" x2="-1.973" y2="-0.983" width="0.0508" layer="39"/>
<wire x1="-1.973" y1="-0.983" x2="-1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="-0.4" y1="0.65" x2="0.4" y2="0.65" width="0.1" layer="51"/>
<wire x1="-0.4" y1="-0.65" x2="0.4" y2="-0.65" width="0.1" layer="51"/>
<smd name="1" x="-0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<smd name="2" x="0.95" y="0" dx="1.3" dy="1.5" layer="1"/>
<text x="0" y="1" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-1" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-1" y1="-0.7" x2="-0.4" y2="0.7" layer="51"/>
<rectangle x1="0.4" y1="-0.7" x2="1" y2="0.7" layer="51"/>
<rectangle x1="-0.1001" y1="-0.4001" x2="0.1001" y2="0.4001" layer="35"/>
</package>
<package name="C1005">
<description>&lt;b&gt;CAPACITOR&lt;/b&gt;</description>
<wire x1="-1.5" y1="0.5" x2="1.5" y2="0.5" width="0.0508" layer="39"/>
<wire x1="1.5" y1="0.5" x2="1.5" y2="-0.5" width="0.0508" layer="39"/>
<wire x1="1.5" y1="-0.5" x2="-1.5" y2="-0.5" width="0.0508" layer="39"/>
<wire x1="-1.5" y1="-0.5" x2="-1.5" y2="0.5" width="0.0508" layer="39"/>
<wire x1="-0.3" y1="0.2" x2="0.3" y2="0.2" width="0.1" layer="51"/>
<wire x1="-0.3" y1="-0.2" x2="0.3" y2="-0.2" width="0.1" layer="51"/>
<smd name="1" x="-0.65" y="0" dx="0.9" dy="0.7" layer="1"/>
<smd name="2" x="0.65" y="0" dx="0.9" dy="0.7" layer="1"/>
<text x="0" y="0.5" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-0.5" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-0.5" y1="-0.25" x2="-0.3" y2="0.25" layer="51"/>
<rectangle x1="0.3" y1="-0.25" x2="0.5" y2="0.25" layer="51"/>
<rectangle x1="-0.1001" y1="-0.4001" x2="0.1001" y2="0.4001" layer="35"/>
</package>
<package name="2X05">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-6.35" y1="-1.905" x2="-5.715" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="-2.54" x2="-3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-3.175" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="-2.54" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="-0.635" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-2.54" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="1.905" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="3.175" y1="-2.54" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="-1.905" x2="-6.35" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="1.905" x2="-5.715" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-5.715" y1="2.54" x2="-4.445" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="2.54" x2="-3.81" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="1.905" x2="-3.175" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="2.54" x2="-1.905" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="2.54" x2="-1.27" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="1.905" x2="-0.635" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="2.54" x2="0.635" y2="2.54" width="0.1524" layer="21"/>
<wire x1="0.635" y1="2.54" x2="1.27" y2="1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="1.905" x2="1.905" y2="2.54" width="0.1524" layer="21"/>
<wire x1="1.905" y1="2.54" x2="3.175" y2="2.54" width="0.1524" layer="21"/>
<wire x1="3.175" y1="2.54" x2="3.81" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="1.905" x2="-3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="1.905" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="1.905" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="1.905" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.905" y1="-2.54" x2="3.175" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-2.54" x2="0.635" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="-2.54" x2="-1.905" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-5.715" y1="-2.54" x2="-4.445" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="4.445" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="5.715" y1="-2.54" x2="6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="1.905" x2="4.445" y2="2.54" width="0.1524" layer="21"/>
<wire x1="4.445" y1="2.54" x2="5.715" y2="2.54" width="0.1524" layer="21"/>
<wire x1="5.715" y1="2.54" x2="6.35" y2="1.905" width="0.1524" layer="21"/>
<wire x1="6.35" y1="1.905" x2="6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="4.445" y1="-2.54" x2="5.715" y2="-2.54" width="0.1524" layer="21"/>
<pad name="1" x="-5.08" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="2" x="-5.08" y="1.27" drill="1.016" shape="octagon"/>
<pad name="3" x="-2.54" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="4" x="-2.54" y="1.27" drill="1.016" shape="octagon"/>
<pad name="5" x="0" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="6" x="0" y="1.27" drill="1.016" shape="octagon"/>
<pad name="7" x="2.54" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="8" x="2.54" y="1.27" drill="1.016" shape="octagon"/>
<pad name="9" x="5.08" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="10" x="5.08" y="1.27" drill="1.016" shape="octagon"/>
<text x="0" y="3" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-3" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-5.334" y1="-1.524" x2="-4.826" y2="-1.016" layer="51"/>
<rectangle x1="-5.334" y1="1.016" x2="-4.826" y2="1.524" layer="51"/>
<rectangle x1="-2.794" y1="1.016" x2="-2.286" y2="1.524" layer="51"/>
<rectangle x1="-2.794" y1="-1.524" x2="-2.286" y2="-1.016" layer="51"/>
<rectangle x1="-0.254" y1="1.016" x2="0.254" y2="1.524" layer="51"/>
<rectangle x1="-0.254" y1="-1.524" x2="0.254" y2="-1.016" layer="51"/>
<rectangle x1="2.286" y1="1.016" x2="2.794" y2="1.524" layer="51"/>
<rectangle x1="2.286" y1="-1.524" x2="2.794" y2="-1.016" layer="51"/>
<rectangle x1="4.826" y1="1.016" x2="5.334" y2="1.524" layer="51"/>
<rectangle x1="4.826" y1="-1.524" x2="5.334" y2="-1.016" layer="51"/>
</package>
<package name="2X05/90">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-6.35" y1="-1.905" x2="-3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="0.635" x2="-6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-5.08" y1="6.985" x2="-5.08" y2="1.27" width="0.762" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="6.985" x2="-2.54" y2="1.27" width="0.762" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="6.985" x2="0" y2="1.27" width="0.762" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="0.635" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="6.985" x2="2.54" y2="1.27" width="0.762" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="6.35" y1="-1.905" x2="6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="6.35" y1="0.635" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="5.08" y1="6.985" x2="5.08" y2="1.27" width="0.762" layer="21"/>
<pad name="2" x="-5.08" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="4" x="-2.54" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="6" x="0" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="8" x="2.54" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="10" x="5.08" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="1" x="-5.08" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="3" x="-2.54" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="5" x="0" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="7" x="2.54" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="9" x="5.08" y="-6.35" drill="1.016" shape="octagon"/>
<text x="-6.985" y="-3.81" size="1.27" layer="25" ratio="10" rot="R90">&gt;NAME</text>
<text x="8.255" y="-3.81" size="1.27" layer="27" rot="R90">&gt;VALUE</text>
<rectangle x1="-5.461" y1="0.635" x2="-4.699" y2="1.143" layer="21"/>
<rectangle x1="-2.921" y1="0.635" x2="-2.159" y2="1.143" layer="21"/>
<rectangle x1="-0.381" y1="0.635" x2="0.381" y2="1.143" layer="21"/>
<rectangle x1="2.159" y1="0.635" x2="2.921" y2="1.143" layer="21"/>
<rectangle x1="4.699" y1="0.635" x2="5.461" y2="1.143" layer="21"/>
<rectangle x1="-5.461" y1="-2.921" x2="-4.699" y2="-1.905" layer="21"/>
<rectangle x1="-2.921" y1="-2.921" x2="-2.159" y2="-1.905" layer="21"/>
<rectangle x1="-5.461" y1="-5.461" x2="-4.699" y2="-4.699" layer="21"/>
<rectangle x1="-5.461" y1="-4.699" x2="-4.699" y2="-2.921" layer="51"/>
<rectangle x1="-2.921" y1="-4.699" x2="-2.159" y2="-2.921" layer="51"/>
<rectangle x1="-2.921" y1="-5.461" x2="-2.159" y2="-4.699" layer="21"/>
<rectangle x1="-0.381" y1="-2.921" x2="0.381" y2="-1.905" layer="21"/>
<rectangle x1="2.159" y1="-2.921" x2="2.921" y2="-1.905" layer="21"/>
<rectangle x1="-0.381" y1="-5.461" x2="0.381" y2="-4.699" layer="21"/>
<rectangle x1="-0.381" y1="-4.699" x2="0.381" y2="-2.921" layer="51"/>
<rectangle x1="2.159" y1="-4.699" x2="2.921" y2="-2.921" layer="51"/>
<rectangle x1="2.159" y1="-5.461" x2="2.921" y2="-4.699" layer="21"/>
<rectangle x1="4.699" y1="-2.921" x2="5.461" y2="-1.905" layer="21"/>
<rectangle x1="4.699" y1="-5.461" x2="5.461" y2="-4.699" layer="21"/>
<rectangle x1="4.699" y1="-4.699" x2="5.461" y2="-2.921" layer="51"/>
</package>
<package name="2X03">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-3.81" y1="-1.905" x2="-3.175" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="-2.54" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="-0.635" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-2.54" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-3.81" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="1.905" x2="-3.175" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="2.54" x2="-1.905" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="2.54" x2="-1.27" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="1.905" x2="-0.635" y2="2.54" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="2.54" x2="0.635" y2="2.54" width="0.1524" layer="21"/>
<wire x1="0.635" y1="2.54" x2="1.27" y2="1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="1.905" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="1.905" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-2.54" x2="0.635" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="-2.54" x2="-1.905" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="1.905" y2="-2.54" width="0.1524" layer="21"/>
<wire x1="3.175" y1="-2.54" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="1.905" x2="1.905" y2="2.54" width="0.1524" layer="21"/>
<wire x1="1.905" y1="2.54" x2="3.175" y2="2.54" width="0.1524" layer="21"/>
<wire x1="3.175" y1="2.54" x2="3.81" y2="1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="1.905" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.905" y1="-2.54" x2="3.175" y2="-2.54" width="0.1524" layer="21"/>
<pad name="1" x="-2.54" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="2" x="-2.54" y="1.27" drill="1.016" shape="octagon"/>
<pad name="3" x="0" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="4" x="0" y="1.27" drill="1.016" shape="octagon"/>
<pad name="5" x="2.54" y="-1.27" drill="1.016" shape="octagon"/>
<pad name="6" x="2.54" y="1.27" drill="1.016" shape="octagon"/>
<text x="0" y="3" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-3" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-2.794" y1="-1.524" x2="-2.286" y2="-1.016" layer="51"/>
<rectangle x1="-2.794" y1="1.016" x2="-2.286" y2="1.524" layer="51"/>
<rectangle x1="-0.254" y1="1.016" x2="0.254" y2="1.524" layer="51"/>
<rectangle x1="-0.254" y1="-1.524" x2="0.254" y2="-1.016" layer="51"/>
<rectangle x1="2.286" y1="1.016" x2="2.794" y2="1.524" layer="51"/>
<rectangle x1="2.286" y1="-1.524" x2="2.794" y2="-1.016" layer="51"/>
</package>
<package name="1X05">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="1.905" y1="1.27" x2="3.175" y2="1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="1.27" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="0.635" x2="3.81" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-0.635" x2="3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="1.27" x2="0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="1.27" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-0.635" x2="0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-1.27" x2="-0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-1.27" x2="-1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="1.905" y1="1.27" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-0.635" x2="1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="-1.27" x2="1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-5.715" y1="1.27" x2="-4.445" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="1.27" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-3.81" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-0.635" x2="-4.445" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-3.175" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="1.27" x2="-1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="1.27" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-0.635" x2="-1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="-1.27" x2="-3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="-1.27" x2="-3.81" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="0.635" x2="-6.35" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-5.715" y1="1.27" x2="-6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="-0.635" x2="-5.715" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="-1.27" x2="-5.715" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="4.445" y1="1.27" x2="5.715" y2="1.27" width="0.1524" layer="21"/>
<wire x1="5.715" y1="1.27" x2="6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="6.35" y1="0.635" x2="6.35" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="6.35" y1="-0.635" x2="5.715" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="4.445" y1="1.27" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-0.635" x2="4.445" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="5.715" y1="-1.27" x2="4.445" y2="-1.27" width="0.1524" layer="21"/>
<pad name="1" x="-5.08" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="-2.54" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="3" x="0" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="4" x="2.54" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="5" x="5.08" y="0" drill="1.016" shape="long" rot="R90"/>
<text x="0" y="2" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="2.286" y1="-0.254" x2="2.794" y2="0.254" layer="51"/>
<rectangle x1="-0.254" y1="-0.254" x2="0.254" y2="0.254" layer="51"/>
<rectangle x1="-2.794" y1="-0.254" x2="-2.286" y2="0.254" layer="51"/>
<rectangle x1="-5.334" y1="-0.254" x2="-4.826" y2="0.254" layer="51"/>
<rectangle x1="4.826" y1="-0.254" x2="5.334" y2="0.254" layer="51"/>
</package>
<package name="HC49/S">
<description>&lt;b&gt;CRYSTAL&lt;/b&gt;</description>
<wire x1="-3" y1="-2.15" x2="3" y2="-2.15" width="0.4064" layer="21"/>
<wire x1="-3" y1="2.15" x2="3" y2="2.15" width="0.4064" layer="21"/>
<wire x1="-3" y1="-1.65" x2="3" y2="-1.65" width="0.1524" layer="21"/>
<wire x1="3" y1="1.65" x2="-3" y2="1.65" width="0.1524" layer="21"/>
<wire x1="-0.25" y1="0.75" x2="0.25" y2="0.75" width="0.1524" layer="21"/>
<wire x1="0.25" y1="0.75" x2="0.25" y2="-0.75" width="0.1524" layer="21"/>
<wire x1="0.25" y1="-0.75" x2="-0.25" y2="-0.75" width="0.1524" layer="21"/>
<wire x1="-0.25" y1="-0.75" x2="-0.25" y2="0.75" width="0.1524" layer="21"/>
<wire x1="0.65" y1="0.75" x2="0.65" y2="0" width="0.1524" layer="21"/>
<wire x1="0.65" y1="0" x2="0.65" y2="-0.75" width="0.1524" layer="21"/>
<wire x1="-0.65" y1="0.75" x2="-0.65" y2="0" width="0.1524" layer="21"/>
<wire x1="-0.65" y1="0" x2="-0.65" y2="-0.75" width="0.1524" layer="21"/>
<wire x1="0.65" y1="0" x2="1.3" y2="0" width="0.1524" layer="21"/>
<wire x1="-0.65" y1="0" x2="-1.3" y2="0" width="0.1524" layer="21"/>
<wire x1="-3" y1="2.15" x2="-3" y2="-2.15" width="0.4064" layer="21" curve="180"/>
<wire x1="3" y1="-2.15" x2="3" y2="2.15" width="0.4064" layer="21" curve="180"/>
<wire x1="-3" y1="1.65" x2="-3" y2="-1.65" width="0.1524" layer="21" curve="180"/>
<wire x1="3" y1="-1.65" x2="3" y2="1.65" width="0.1524" layer="21" curve="180"/>
<pad name="1" x="-2.44" y="0" drill="0.8128"/>
<pad name="2" x="2.44" y="0" drill="0.8128"/>
<text x="0" y="2.5" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2.5" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-4.445" y1="-2.54" x2="4.445" y2="2.54" layer="43"/>
<rectangle x1="-5.08" y1="-1.905" x2="-4.445" y2="1.905" layer="43"/>
<rectangle x1="-5.715" y1="-1.27" x2="-5.08" y2="1.27" layer="43"/>
<rectangle x1="4.445" y1="-1.905" x2="5.08" y2="1.905" layer="43"/>
<rectangle x1="5.08" y1="-1.27" x2="5.715" y2="1.27" layer="43"/>
</package>
<package name="HC-49US">
<smd name="1" x="-4.75" y="0" dx="5.5" dy="2" layer="1"/>
<smd name="2" x="4.75" y="0" dx="5.5" dy="2" layer="1"/>
<wire x1="-6.5" y1="2.5" x2="6.5" y2="2.5" width="0.127" layer="51"/>
<wire x1="6.5" y1="2.5" x2="6.5" y2="-2.5" width="0.127" layer="51"/>
<wire x1="6.5" y1="-2.5" x2="-6.5" y2="-2.5" width="0.127" layer="51"/>
<wire x1="-6.5" y1="-2.5" x2="-6.5" y2="2.5" width="0.127" layer="51"/>
<text x="0" y="1.5" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-1.5" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<wire x1="-8" y1="3.5" x2="-8" y2="-3.5" width="0.127" layer="39"/>
<wire x1="-8" y1="-3.5" x2="8" y2="-3.5" width="0.127" layer="39"/>
<wire x1="8" y1="-3.5" x2="8" y2="3.5" width="0.127" layer="39"/>
<wire x1="8" y1="3.5" x2="-8" y2="3.5" width="0.127" layer="39"/>
<wire x1="-1.5" y1="0" x2="-1" y2="0" width="0.127" layer="21"/>
<wire x1="-1" y1="0" x2="-1" y2="1" width="0.127" layer="21"/>
<wire x1="-1" y1="0" x2="-1" y2="-1" width="0.127" layer="21"/>
<wire x1="-0.5" y1="1" x2="-0.5" y2="-1" width="0.127" layer="21"/>
<wire x1="-0.5" y1="-1" x2="0.5" y2="-1" width="0.127" layer="21"/>
<wire x1="0.5" y1="-1" x2="0.5" y2="1" width="0.127" layer="21"/>
<wire x1="0.5" y1="1" x2="-0.5" y2="1" width="0.127" layer="21"/>
<wire x1="1" y1="1" x2="1" y2="0" width="0.127" layer="21"/>
<wire x1="1" y1="0" x2="1" y2="-1" width="0.127" layer="21"/>
<wire x1="1" y1="0" x2="1.5" y2="0" width="0.127" layer="21"/>
</package>
<package name="R1608">
<description>&lt;b&gt;RESISTOR&lt;/b&gt;</description>
<wire x1="-1.5" y1="1" x2="1.5" y2="1" width="0.0508" layer="39"/>
<wire x1="1.5" y1="1" x2="1.5" y2="-1" width="0.0508" layer="39"/>
<wire x1="1.5" y1="-1" x2="-1.5" y2="-1" width="0.0508" layer="39"/>
<wire x1="-1.5" y1="-1" x2="-1.5" y2="1" width="0.0508" layer="39"/>
<wire x1="-0.4" y1="0.4" x2="0.4" y2="0.4" width="0.1" layer="51"/>
<wire x1="-0.4" y1="-0.4" x2="0.4" y2="-0.4" width="0.1" layer="51"/>
<smd name="1" x="-0.85" y="0" dx="1.1" dy="1" layer="1"/>
<smd name="2" x="0.85" y="0" dx="1.1" dy="1" layer="1"/>
<text x="0" y="0.75" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-0.75" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-0.8" y1="-0.45" x2="-0.4" y2="0.45" layer="51"/>
<rectangle x1="0.4" y1="-0.45" x2="0.8" y2="0.45" layer="51"/>
<rectangle x1="-0.1001" y1="-0.4001" x2="0.1001" y2="0.4001" layer="35"/>
</package>
<package name="TSSOP14">
<description>&lt;b&gt;plastic thin shrink small outline package; 14 leads; body width 4.4 mm&lt;/b&gt;&lt;p&gt;
SOT402-1&lt;br&gt;
Source: http://www.nxp.com/documents/data_sheet/74ABT125.pdf</description>
<wire x1="-2.5" y1="2.2" x2="2.5" y2="2.2" width="0.2032" layer="21"/>
<wire x1="2.5" y1="2.2" x2="2.5" y2="-2.2" width="0.2032" layer="21"/>
<wire x1="2.5" y1="-2.2" x2="-2.5" y2="-2.2" width="0.2032" layer="21"/>
<wire x1="-2.5" y1="-2.2" x2="-2.5" y2="2.2" width="0.2032" layer="21"/>
<circle x="-1.6" y="-1.3" radius="0.5" width="0" layer="21"/>
<smd name="1" x="-1.95" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="2" x="-1.3" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="3" x="-0.65" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="4" x="0" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="5" x="0.65" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="6" x="1.3" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="7" x="1.95" y="-3.05" dx="0.45" dy="1.65" layer="1" stop="no"/>
<smd name="8" x="1.95" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="9" x="1.3" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="10" x="0.65" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="11" x="0" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="12" x="-0.65" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="13" x="-1.3" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<smd name="14" x="-1.95" y="3.05" dx="0.45" dy="1.65" layer="1" rot="R180" stop="no"/>
<text x="-3" y="-2" size="0.6" layer="25" font="vector" ratio="20" rot="R90">&gt;NAME</text>
<text x="3" y="-2" size="0.6" layer="27" font="vector" ratio="20" rot="R90" align="top-left">&gt;VALUE</text>
<rectangle x1="-2.1" y1="-3.3" x2="-1.8" y2="-2.225" layer="51"/>
<rectangle x1="-1.45" y1="-3.3" x2="-1.15" y2="-2.225" layer="51"/>
<rectangle x1="-0.8" y1="-3.3" x2="-0.5" y2="-2.225" layer="51"/>
<rectangle x1="-0.15" y1="-3.3" x2="0.15" y2="-2.225" layer="51"/>
<rectangle x1="0.5" y1="-3.3" x2="0.8" y2="-2.225" layer="51"/>
<rectangle x1="1.15" y1="-3.3" x2="1.45" y2="-2.225" layer="51"/>
<rectangle x1="1.8" y1="-3.3" x2="2.1" y2="-2.225" layer="51"/>
<rectangle x1="1.8" y1="2.225" x2="2.1" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="1.15" y1="2.225" x2="1.45" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="0.5" y1="2.225" x2="0.8" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="-0.15" y1="2.225" x2="0.15" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="-0.8" y1="2.225" x2="-0.5" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="-1.45" y1="2.225" x2="-1.15" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="-2.1" y1="2.225" x2="-1.8" y2="3.3" layer="51" rot="R180"/>
<rectangle x1="-2.175" y1="-3.425" x2="-1.725" y2="-2.425" layer="29"/>
<rectangle x1="-1.525" y1="-3.425" x2="-1.075" y2="-2.425" layer="29"/>
<rectangle x1="-0.875" y1="-3.425" x2="-0.425" y2="-2.425" layer="29"/>
<rectangle x1="-0.225" y1="-3.425" x2="0.225" y2="-2.425" layer="29"/>
<rectangle x1="0.425" y1="-3.425" x2="0.875" y2="-2.425" layer="29"/>
<rectangle x1="1.075" y1="-3.425" x2="1.525" y2="-2.425" layer="29"/>
<rectangle x1="1.725" y1="-3.425" x2="2.175" y2="-2.425" layer="29"/>
<rectangle x1="1.725" y1="2.425" x2="2.175" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="1.075" y1="2.425" x2="1.525" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="0.425" y1="2.425" x2="0.875" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="-0.225" y1="2.425" x2="0.225" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="-0.875" y1="2.425" x2="-0.425" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="-1.525" y1="2.425" x2="-1.075" y2="3.425" layer="29" rot="R180"/>
<rectangle x1="-2.175" y1="2.425" x2="-1.725" y2="3.425" layer="29" rot="R180"/>
</package>
<package name="1X04">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="0" y1="0.635" x2="0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="1.27" x2="1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="1.27" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="0.635" x2="2.54" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="-0.635" x2="1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="-1.27" x2="0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-1.27" x2="0" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="1.27" x2="-3.175" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="1.27" x2="-2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="0.635" x2="-2.54" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="-0.635" x2="-3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="0.635" x2="-1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="1.27" x2="-0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="1.27" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="0.635" x2="0" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="-0.635" x2="-0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-1.27" x2="-1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="-1.27" x2="-2.54" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-5.08" y1="0.635" x2="-5.08" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-4.445" y1="1.27" x2="-5.08" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-5.08" y1="-0.635" x2="-4.445" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="-1.27" x2="-4.445" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="1.27" x2="4.445" y2="1.27" width="0.1524" layer="21"/>
<wire x1="4.445" y1="1.27" x2="5.08" y2="0.635" width="0.1524" layer="21"/>
<wire x1="5.08" y1="0.635" x2="5.08" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="5.08" y1="-0.635" x2="4.445" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="1.27" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="-0.635" x2="3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="4.445" y1="-1.27" x2="3.175" y2="-1.27" width="0.1524" layer="21"/>
<pad name="1" x="-3.81" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="-1.27" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="3" x="1.27" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="4" x="3.81" y="0" drill="1.016" shape="long" rot="R90"/>
<text x="0" y="2" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="1.016" y1="-0.254" x2="1.524" y2="0.254" layer="51"/>
<rectangle x1="-1.524" y1="-0.254" x2="-1.016" y2="0.254" layer="51"/>
<rectangle x1="-4.064" y1="-0.254" x2="-3.556" y2="0.254" layer="51"/>
<rectangle x1="3.556" y1="-0.254" x2="4.064" y2="0.254" layer="51"/>
</package>
<package name="1X04/90">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-5.08" y1="-1.905" x2="-2.54" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="-1.905" x2="-2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="0.635" x2="-5.08" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-5.08" y1="0.635" x2="-5.08" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="6.985" x2="-3.81" y2="1.27" width="0.762" layer="21"/>
<wire x1="-2.54" y1="-1.905" x2="0" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="0" y1="-1.905" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="0.635" x2="-2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="6.985" x2="-1.27" y2="1.27" width="0.762" layer="21"/>
<wire x1="0" y1="-1.905" x2="2.54" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="2.54" y1="-1.905" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="0.635" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="6.985" x2="1.27" y2="1.27" width="0.762" layer="21"/>
<wire x1="2.54" y1="-1.905" x2="5.08" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="5.08" y1="-1.905" x2="5.08" y2="0.635" width="0.1524" layer="21"/>
<wire x1="5.08" y1="0.635" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="6.985" x2="3.81" y2="1.27" width="0.762" layer="21"/>
<pad name="1" x="-3.81" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="-1.27" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="3" x="1.27" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="4" x="3.81" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<text x="-5.715" y="-3.81" size="1.27" layer="25" ratio="10" rot="R90">&gt;NAME</text>
<text x="6.985" y="-4.445" size="1.27" layer="27" rot="R90">&gt;VALUE</text>
<rectangle x1="-4.191" y1="0.635" x2="-3.429" y2="1.143" layer="21"/>
<rectangle x1="-1.651" y1="0.635" x2="-0.889" y2="1.143" layer="21"/>
<rectangle x1="0.889" y1="0.635" x2="1.651" y2="1.143" layer="21"/>
<rectangle x1="3.429" y1="0.635" x2="4.191" y2="1.143" layer="21"/>
<rectangle x1="-4.191" y1="-2.921" x2="-3.429" y2="-1.905" layer="21"/>
<rectangle x1="-1.651" y1="-2.921" x2="-0.889" y2="-1.905" layer="21"/>
<rectangle x1="0.889" y1="-2.921" x2="1.651" y2="-1.905" layer="21"/>
<rectangle x1="3.429" y1="-2.921" x2="4.191" y2="-1.905" layer="21"/>
</package>
<package name="PUSH">
<text x="0" y="3" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-3" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<pad name="1" x="-3" y="2" drill="0.8" shape="long"/>
<pad name="2" x="3" y="2" drill="0.8" shape="long"/>
<pad name="3" x="-3" y="-2" drill="0.8" shape="long"/>
<pad name="4" x="3" y="-2" drill="0.8" shape="long"/>
<wire x1="-2.5" y1="-2.5" x2="-2.5" y2="2.5" width="0.4064" layer="21"/>
<wire x1="-2.5" y1="2.5" x2="2.5" y2="2.5" width="0.4064" layer="21"/>
<wire x1="2.5" y1="2.5" x2="2.5" y2="-2.5" width="0.4064" layer="21"/>
<wire x1="2.5" y1="-2.5" x2="-2.5" y2="-2.5" width="0.4064" layer="21"/>
</package>
<package name="2X03/90">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-3.81" y1="-1.905" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="6.985" x2="-2.54" y2="1.27" width="0.762" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="6.985" x2="0" y2="1.27" width="0.762" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="0.635" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="6.985" x2="2.54" y2="1.27" width="0.762" layer="21"/>
<pad name="2" x="-2.54" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="4" x="0" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="6" x="2.54" y="-3.81" drill="1.016" shape="octagon"/>
<pad name="1" x="-2.54" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="3" x="0" y="-6.35" drill="1.016" shape="octagon"/>
<pad name="5" x="2.54" y="-6.35" drill="1.016" shape="octagon"/>
<text x="-4.445" y="-3.81" size="1.27" layer="25" ratio="10" rot="R90">&gt;NAME</text>
<text x="5.715" y="-3.81" size="1.27" layer="27" rot="R90">&gt;VALUE</text>
<rectangle x1="-2.921" y1="0.635" x2="-2.159" y2="1.143" layer="21"/>
<rectangle x1="-0.381" y1="0.635" x2="0.381" y2="1.143" layer="21"/>
<rectangle x1="2.159" y1="0.635" x2="2.921" y2="1.143" layer="21"/>
<rectangle x1="-2.921" y1="-2.921" x2="-2.159" y2="-1.905" layer="21"/>
<rectangle x1="-0.381" y1="-2.921" x2="0.381" y2="-1.905" layer="21"/>
<rectangle x1="-2.921" y1="-5.461" x2="-2.159" y2="-4.699" layer="21"/>
<rectangle x1="-2.921" y1="-4.699" x2="-2.159" y2="-2.921" layer="51"/>
<rectangle x1="-0.381" y1="-4.699" x2="0.381" y2="-2.921" layer="51"/>
<rectangle x1="-0.381" y1="-5.461" x2="0.381" y2="-4.699" layer="21"/>
<rectangle x1="2.159" y1="-2.921" x2="2.921" y2="-1.905" layer="21"/>
<rectangle x1="2.159" y1="-5.461" x2="2.921" y2="-4.699" layer="21"/>
<rectangle x1="2.159" y1="-4.699" x2="2.921" y2="-2.921" layer="51"/>
</package>
<package name="1X02">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-1.905" y1="1.27" x2="-0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="1.27" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="0.635" x2="0" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="-0.635" x2="-0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="0.635" x2="-2.54" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="1.27" x2="-2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="-0.635" x2="-1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-1.27" x2="-1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="0" y1="0.635" x2="0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="1.27" x2="1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="1.27" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="0.635" x2="2.54" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="-0.635" x2="1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="-1.27" x2="0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-1.27" x2="0" y2="-0.635" width="0.1524" layer="21"/>
<pad name="1" x="-1.27" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="1.27" y="0" drill="1.016" shape="long" rot="R90"/>
<text x="0" y="2" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-1.524" y1="-0.254" x2="-1.016" y2="0.254" layer="51"/>
<rectangle x1="1.016" y1="-0.254" x2="1.524" y2="0.254" layer="51"/>
</package>
<package name="1X02/90">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-2.54" y1="-1.905" x2="0" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="0" y1="-1.905" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="0.635" x2="-2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="0.635" x2="-2.54" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="6.985" x2="-1.27" y2="1.27" width="0.762" layer="21"/>
<wire x1="0" y1="-1.905" x2="2.54" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="2.54" y1="-1.905" x2="2.54" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="0.635" x2="0" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="6.985" x2="1.27" y2="1.27" width="0.762" layer="21"/>
<pad name="1" x="-1.27" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="1.27" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<text x="-3.175" y="-3.81" size="1.27" layer="25" ratio="10" rot="R90">&gt;NAME</text>
<text x="4.445" y="-3.81" size="1.27" layer="27" rot="R90">&gt;VALUE</text>
<rectangle x1="-1.651" y1="0.635" x2="-0.889" y2="1.143" layer="21"/>
<rectangle x1="0.889" y1="0.635" x2="1.651" y2="1.143" layer="21"/>
<rectangle x1="-1.651" y1="-2.921" x2="-0.889" y2="-1.905" layer="21"/>
<rectangle x1="0.889" y1="-2.921" x2="1.651" y2="-1.905" layer="21"/>
</package>
<package name="2,8-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 2.8 mm, round</description>
<wire x1="0" y1="2.921" x2="0" y2="2.667" width="0.0508" layer="21"/>
<wire x1="0" y1="-2.667" x2="0" y2="-2.921" width="0.0508" layer="21"/>
<wire x1="-1.778" y1="0" x2="0" y2="-1.778" width="2.286" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="1.778" x2="1.778" y2="0" width="2.286" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="0.635" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="2.921" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="3.175" width="0.8128" layer="39"/>
<circle x="0" y="0" radius="3.175" width="0.8128" layer="40"/>
<circle x="0" y="0" radius="3.175" width="0.8128" layer="43"/>
<circle x="0" y="0" radius="1.5" width="0.2032" layer="21"/>
<pad name="B2,8" x="0" y="0" drill="2.8" diameter="5.334"/>
</package>
<package name="3,0-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 3.0 mm, round</description>
<wire x1="-2.159" y1="0" x2="0" y2="-2.159" width="2.4892" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.159" x2="2.159" y2="0" width="2.4892" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="3.429" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="39"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="40"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="43"/>
<circle x="0" y="0" radius="1.6" width="0.2032" layer="21"/>
<pad name="B3,0" x="0" y="0" drill="3" diameter="5.842"/>
<text x="-1.27" y="-3.81" size="1.27" layer="48">3,0</text>
</package>
<package name="3,2-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 3.2 mm, round</description>
<wire x1="-2.159" y1="0" x2="0" y2="-2.159" width="2.4892" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.159" x2="2.159" y2="0" width="2.4892" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="3.429" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="3.683" width="1.27" layer="39"/>
<circle x="0" y="0" radius="3.683" width="1.27" layer="40"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="43"/>
<circle x="0" y="0" radius="1.7" width="0.1524" layer="21"/>
<pad name="B3,2" x="0" y="0" drill="3.2" diameter="5.842"/>
<text x="-1.27" y="-3.81" size="1.27" layer="48">3,2</text>
</package>
<package name="3,3-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 3.3 mm, round</description>
<wire x1="-2.159" y1="0" x2="0" y2="-2.159" width="2.4892" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.159" x2="2.159" y2="0" width="2.4892" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="3.429" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="3.683" width="1.27" layer="39"/>
<circle x="0" y="0" radius="3.683" width="1.27" layer="40"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="43"/>
<circle x="0" y="0" radius="1.7" width="0.2032" layer="21"/>
<pad name="B3,3" x="0" y="0" drill="3.3" diameter="5.842"/>
</package>
<package name="3,6-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 3.6 mm, round</description>
<wire x1="-2.159" y1="0" x2="0" y2="-2.159" width="2.4892" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.159" x2="2.159" y2="0" width="2.4892" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="3.429" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="3.683" width="1.397" layer="39"/>
<circle x="0" y="0" radius="3.683" width="1.397" layer="40"/>
<circle x="0" y="0" radius="3.556" width="1.016" layer="43"/>
<circle x="0" y="0" radius="1.9" width="0.2032" layer="21"/>
<pad name="B3,6" x="0" y="0" drill="3.6" diameter="5.842"/>
</package>
<package name="4,1-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 4.1 mm, round</description>
<wire x1="-2.54" y1="0" x2="0" y2="-2.54" width="3.9116" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.54" x2="2.54" y2="0" width="3.9116" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="4.572" width="0.1524" layer="21"/>
<circle x="0" y="0" radius="5.08" width="2" layer="43"/>
<circle x="0" y="0" radius="2.15" width="0.2032" layer="21"/>
<pad name="B4,1" x="0" y="0" drill="4.1" diameter="8"/>
</package>
<package name="4,3-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 4.3 mm, round</description>
<wire x1="-2.54" y1="0" x2="0" y2="-2.54" width="3.9116" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.54" x2="2.54" y2="0" width="3.9116" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="4.4958" width="0.1524" layer="51"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="5.588" width="2" layer="43"/>
<circle x="0" y="0" radius="5.588" width="2" layer="39"/>
<circle x="0" y="0" radius="5.588" width="2" layer="40"/>
<circle x="0" y="0" radius="2.25" width="0.1524" layer="21"/>
<pad name="B4,3" x="0" y="0" drill="4.3" diameter="9"/>
</package>
<package name="4,5-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 4.5 mm, round</description>
<wire x1="-2.54" y1="0" x2="0" y2="-2.54" width="3.9116" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.54" x2="2.54" y2="0" width="3.9116" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="4.4958" width="0.1524" layer="51"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="5.588" width="2" layer="43"/>
<circle x="0" y="0" radius="5.588" width="2" layer="39"/>
<circle x="0" y="0" radius="5.588" width="2" layer="40"/>
<circle x="0" y="0" radius="2.35" width="0.1524" layer="21"/>
<pad name="B4,5" x="0" y="0" drill="4.5" diameter="9"/>
</package>
<package name="5,0-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 5.0 mm, round</description>
<wire x1="-2.54" y1="0" x2="0" y2="-2.54" width="3.9116" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.54" x2="2.54" y2="0" width="3.9116" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="4.4958" width="0.1524" layer="51"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="5.588" width="2" layer="43"/>
<circle x="0" y="0" radius="5.588" width="2" layer="39"/>
<circle x="0" y="0" radius="5.588" width="2" layer="40"/>
<circle x="0" y="0" radius="2.6" width="0.1524" layer="21"/>
<pad name="B5" x="0" y="0" drill="5" diameter="9"/>
</package>
<package name="5,5-PAD">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt; 5.5 mm, round</description>
<wire x1="-2.54" y1="0" x2="0" y2="-2.54" width="3.9116" layer="51" curve="90" cap="flat"/>
<wire x1="0" y1="2.54" x2="2.54" y2="0" width="3.9116" layer="51" curve="-90" cap="flat"/>
<circle x="0" y="0" radius="4.4958" width="0.1524" layer="51"/>
<circle x="0" y="0" radius="0.762" width="0.4572" layer="51"/>
<circle x="0" y="0" radius="5.588" width="2" layer="43"/>
<circle x="0" y="0" radius="5.588" width="2" layer="39"/>
<circle x="0" y="0" radius="5.588" width="2" layer="40"/>
<circle x="0" y="0" radius="2.85" width="0.1524" layer="21"/>
<pad name="B5,5" x="0" y="0" drill="5.5" diameter="9"/>
</package>
<package name="SOT23-5L">
<description>&lt;b&gt;Small Outline Transistor&lt;/b&gt;&lt;p&gt;
package type OT</description>
<wire x1="1.45" y1="0.8" x2="1.45" y2="-0.8" width="0.1524" layer="21"/>
<wire x1="1.45" y1="-0.8" x2="-1.45" y2="-0.8" width="0.1524" layer="51"/>
<wire x1="-1.45" y1="-0.8" x2="-1.45" y2="0.8" width="0.1524" layer="21"/>
<wire x1="-1.45" y1="0.8" x2="1.45" y2="0.8" width="0.1524" layer="51"/>
<smd name="1" x="-0.95" y="-1.2" dx="0.6" dy="1" layer="1"/>
<smd name="2" x="0" y="-1.2" dx="0.6" dy="1" layer="1"/>
<smd name="3" x="0.95" y="-1.2" dx="0.6" dy="1" layer="1"/>
<smd name="4" x="0.95" y="1.2" dx="0.6" dy="1" layer="1"/>
<smd name="5" x="-0.95" y="1.2" dx="0.6" dy="1" layer="1"/>
<text x="0" y="2" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-1.2" y1="-1.5" x2="-0.7" y2="-0.85" layer="51"/>
<rectangle x1="-0.25" y1="-1.5" x2="0.25" y2="-0.85" layer="51"/>
<rectangle x1="0.7" y1="-1.5" x2="1.2" y2="-0.85" layer="51"/>
<rectangle x1="0.7" y1="0.85" x2="1.2" y2="1.5" layer="51"/>
<rectangle x1="-1.2" y1="0.85" x2="-0.7" y2="1.5" layer="51"/>
</package>
<package name="5-103361-1">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-3.175" y1="1.27" x2="-1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="1.27" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-0.635" x2="-1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="1.27" x2="0.635" y2="1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="1.27" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-0.635" x2="0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="0.635" y1="-1.27" x2="-0.635" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-0.635" y1="-1.27" x2="-1.27" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-3.81" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="-3.175" y1="1.27" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-0.635" x2="-3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="-1.905" y1="-1.27" x2="-3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="1.905" y2="1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="1.27" x2="3.175" y2="1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="1.27" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="0.635" x2="3.81" y2="-0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-0.635" x2="3.175" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="3.175" y1="-1.27" x2="1.905" y2="-1.27" width="0.1524" layer="21"/>
<wire x1="1.905" y1="-1.27" x2="1.27" y2="-0.635" width="0.1524" layer="21"/>
<pad name="1" x="-2.54" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="0" y="0" drill="1.016" shape="long" rot="R90"/>
<pad name="3" x="2.54" y="0" drill="1.016" shape="long" rot="R90"/>
<text x="0" y="2" size="1.27" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-2" size="1.27" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-0.254" y1="-0.254" x2="0.254" y2="0.254" layer="51"/>
<rectangle x1="-2.794" y1="-0.254" x2="-2.286" y2="0.254" layer="51"/>
<rectangle x1="2.286" y1="-0.254" x2="2.794" y2="0.254" layer="51"/>
<wire x1="-5.334" y1="12.192" x2="-5.334" y2="-1.778" width="0.127" layer="51"/>
<wire x1="-5.334" y1="-1.778" x2="5.334" y2="-1.778" width="0.127" layer="51"/>
<wire x1="5.334" y1="-1.778" x2="5.334" y2="12.192" width="0.127" layer="51"/>
<wire x1="5.334" y1="12.192" x2="-5.334" y2="12.192" width="0.127" layer="51"/>
</package>
<package name="RTRIMT93YA">
<description>&lt;b&gt;Trimm resistor&lt;/b&gt; VISHAY&lt;p&gt;
Cermet, abgedichtet nach IP67</description>
<wire x1="2.3" y1="-4.75" x2="2.3" y2="4.75" width="0.254" layer="21"/>
<wire x1="2.3" y1="4.75" x2="-2.4" y2="4.75" width="0.254" layer="21"/>
<wire x1="-2.4" y1="4.75" x2="-2.4" y2="-4.75" width="0.254" layer="21"/>
<wire x1="-2.4" y1="-4.75" x2="2.3" y2="-4.75" width="0.254" layer="21"/>
<wire x1="-0.6" y1="2.6" x2="-0.15" y2="3.3" width="0.1524" layer="21" curve="-311.390901"/>
<wire x1="-0.6" y1="2.6" x2="-0.15" y2="3.3" width="0.1524" layer="51" curve="48.609099"/>
<pad name="1" x="0" y="-2.5" drill="0.8"/>
<pad name="2" x="0" y="0" drill="0.8"/>
<pad name="3" x="0" y="2.5" drill="0.8"/>
<text x="-3" y="0" size="1.27" layer="25" font="vector" ratio="20" rot="R90" align="bottom-center">&gt;NAME</text>
<text x="2.5" y="0" size="1.27" layer="27" font="vector" ratio="20" rot="R90" align="top-center">&gt;VALUE</text>
<rectangle x1="-1.3" y1="2.5" x2="-1" y2="4.4" layer="21"/>
</package>
<package name="1X05/90">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<wire x1="-6.35" y1="-1.905" x2="-3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-3.81" y1="0.635" x2="-6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-6.35" y1="0.635" x2="-6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-5.08" y1="6.985" x2="-5.08" y2="1.27" width="0.762" layer="21"/>
<wire x1="-3.81" y1="-1.905" x2="-1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-1.27" y1="0.635" x2="-3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="-2.54" y1="6.985" x2="-2.54" y2="1.27" width="0.762" layer="21"/>
<wire x1="-1.27" y1="-1.905" x2="1.27" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="1.27" y1="0.635" x2="-1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="0" y1="6.985" x2="0" y2="1.27" width="0.762" layer="21"/>
<wire x1="1.27" y1="-1.905" x2="3.81" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="3.81" y1="0.635" x2="1.27" y2="0.635" width="0.1524" layer="21"/>
<wire x1="2.54" y1="6.985" x2="2.54" y2="1.27" width="0.762" layer="21"/>
<wire x1="3.81" y1="-1.905" x2="6.35" y2="-1.905" width="0.1524" layer="21"/>
<wire x1="6.35" y1="-1.905" x2="6.35" y2="0.635" width="0.1524" layer="21"/>
<wire x1="6.35" y1="0.635" x2="3.81" y2="0.635" width="0.1524" layer="21"/>
<wire x1="5.08" y1="6.985" x2="5.08" y2="1.27" width="0.762" layer="21"/>
<pad name="1" x="-5.08" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="2" x="-2.54" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="3" x="0" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="4" x="2.54" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<pad name="5" x="5.08" y="-3.81" drill="1.016" shape="long" rot="R90"/>
<text x="-6.985" y="-3.81" size="1.27" layer="25" ratio="10" rot="R90">&gt;NAME</text>
<text x="8.255" y="-3.81" size="1.27" layer="27" rot="R90">&gt;VALUE</text>
<rectangle x1="-5.461" y1="0.635" x2="-4.699" y2="1.143" layer="21"/>
<rectangle x1="-2.921" y1="0.635" x2="-2.159" y2="1.143" layer="21"/>
<rectangle x1="-0.381" y1="0.635" x2="0.381" y2="1.143" layer="21"/>
<rectangle x1="2.159" y1="0.635" x2="2.921" y2="1.143" layer="21"/>
<rectangle x1="4.699" y1="0.635" x2="5.461" y2="1.143" layer="21"/>
<rectangle x1="-5.461" y1="-2.921" x2="-4.699" y2="-1.905" layer="21"/>
<rectangle x1="-2.921" y1="-2.921" x2="-2.159" y2="-1.905" layer="21"/>
<rectangle x1="-0.381" y1="-2.921" x2="0.381" y2="-1.905" layer="21"/>
<rectangle x1="2.159" y1="-2.921" x2="2.921" y2="-1.905" layer="21"/>
<rectangle x1="4.699" y1="-2.921" x2="5.461" y2="-1.905" layer="21"/>
</package>
<package name="1_05X2MM">
<description>CON-M-1X5-200</description>
<text x="-4.5" y="1.5" size="1.27" layer="25" ratio="10">&gt;NAME</text>
<text x="-4.75" y="-2.75" size="1.27" layer="27">&gt;VALUE</text>
<wire x1="-5" y1="0.5" x2="-4.5" y2="1" width="0.1524" layer="21"/>
<wire x1="-4.5" y1="1" x2="-3.5" y2="1" width="0.1524" layer="21"/>
<wire x1="-3.5" y1="1" x2="-3" y2="0.5" width="0.1524" layer="21"/>
<wire x1="-3" y1="-0.5" x2="-3.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="-3.5" y1="-1" x2="-4.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="-4.5" y1="-1" x2="-5" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="-5" y1="0.5" x2="-5" y2="-0.5" width="0.1524" layer="21"/>
<pad name="1" x="-4" y="0" drill="1.016" diameter="1.3" shape="square" rot="R90"/>
<rectangle x1="-4.254" y1="-0.254" x2="-3.746" y2="0.254" layer="51"/>
<wire x1="-3" y1="0.5" x2="-2.5" y2="1" width="0.1524" layer="21"/>
<wire x1="-2.5" y1="1" x2="-1.5" y2="1" width="0.1524" layer="21"/>
<wire x1="-1.5" y1="1" x2="-1" y2="0.5" width="0.1524" layer="21"/>
<wire x1="-1" y1="-0.5" x2="-1.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="-1.5" y1="-1" x2="-2.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="-2.5" y1="-1" x2="-3" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="-3" y1="0.5" x2="-3" y2="-0.5" width="0.1524" layer="21"/>
<pad name="3" x="0" y="0" drill="1.016" diameter="1.3" rot="R90"/>
<rectangle x1="-2.254" y1="-0.254" x2="-1.746" y2="0.254" layer="51"/>
<wire x1="-1" y1="0.5" x2="-0.5" y2="1" width="0.1524" layer="21"/>
<wire x1="-0.5" y1="1" x2="0.5" y2="1" width="0.1524" layer="21"/>
<wire x1="0.5" y1="1" x2="1" y2="0.5" width="0.1524" layer="21"/>
<wire x1="1" y1="-0.5" x2="0.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="0.5" y1="-1" x2="-0.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="-0.5" y1="-1" x2="-1" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="-1" y1="0.5" x2="-1" y2="-0.5" width="0.1524" layer="21"/>
<pad name="2" x="-2" y="0" drill="1.016" diameter="1.3" rot="R90"/>
<rectangle x1="-0.254" y1="-0.254" x2="0.254" y2="0.254" layer="51"/>
<wire x1="1" y1="0.5" x2="1.5" y2="1" width="0.1524" layer="21"/>
<wire x1="1.5" y1="1" x2="2.5" y2="1" width="0.1524" layer="21"/>
<wire x1="2.5" y1="1" x2="3" y2="0.5" width="0.1524" layer="21"/>
<wire x1="3" y1="-0.5" x2="2.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="2.5" y1="-1" x2="1.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="1.5" y1="-1" x2="1" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="1" y1="0.5" x2="1" y2="-0.5" width="0.1524" layer="21"/>
<pad name="4" x="2" y="0" drill="1.016" diameter="1.3" rot="R90"/>
<rectangle x1="1.746" y1="-0.254" x2="2.254" y2="0.254" layer="51"/>
<wire x1="3" y1="0.5" x2="3.5" y2="1" width="0.1524" layer="21"/>
<wire x1="3.5" y1="1" x2="4.5" y2="1" width="0.1524" layer="21"/>
<wire x1="4.5" y1="1" x2="5" y2="0.5" width="0.1524" layer="21"/>
<wire x1="5" y1="0.5" x2="5" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="5" y1="-0.5" x2="4.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="4.5" y1="-1" x2="3.5" y2="-1" width="0.1524" layer="21"/>
<wire x1="3.5" y1="-1" x2="3" y2="-0.5" width="0.1524" layer="21"/>
<wire x1="3" y1="0.5" x2="3" y2="-0.5" width="0.1524" layer="21"/>
<pad name="5" x="4" y="0" drill="1.016" diameter="1.3" rot="R90"/>
<rectangle x1="3.746" y1="-0.254" x2="4.254" y2="0.254" layer="51"/>
</package>
<package name="LED2012">
<description>&lt;b&gt;CAPACITOR&lt;/b&gt;</description>
<wire x1="-1.973" y1="0.983" x2="1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="0.983" x2="1.973" y2="-0.983" width="0.0508" layer="39"/>
<wire x1="1.973" y1="-0.983" x2="-1.973" y2="-0.983" width="0.0508" layer="39"/>
<wire x1="-1.973" y1="-0.983" x2="-1.973" y2="0.983" width="0.0508" layer="39"/>
<wire x1="-0.4" y1="0.65" x2="0.4" y2="0.65" width="0.1" layer="51"/>
<wire x1="-0.4" y1="-0.65" x2="0.4" y2="-0.65" width="0.1" layer="51"/>
<smd name="A" x="-1.15" y="0" dx="1.1" dy="1.3" layer="1"/>
<smd name="C" x="1.15" y="0" dx="1.1" dy="1.3" layer="1"/>
<text x="0" y="1" size="0.6" layer="25" font="vector" ratio="20" align="bottom-center">&gt;NAME</text>
<text x="0" y="-1" size="0.6" layer="27" font="vector" ratio="20" align="top-center">&gt;VALUE</text>
<rectangle x1="-1" y1="-0.7" x2="-0.4" y2="0.7" layer="51"/>
<rectangle x1="0.4" y1="-0.7" x2="1" y2="0.7" layer="51"/>
<rectangle x1="-0.1001" y1="-0.4001" x2="0.1001" y2="0.4001" layer="35"/>
<circle x="2" y="1" radius="0.125" width="0.25" layer="21"/>
</package>
</packages>
<symbols>
<symbol name="STM32F103C8">
<wire x1="-20.32" y1="30.48" x2="20.32" y2="30.48" width="0.254" layer="94"/>
<wire x1="20.32" y1="30.48" x2="20.32" y2="-33.02" width="0.254" layer="94"/>
<wire x1="20.32" y1="-33.02" x2="-20.32" y2="-33.02" width="0.254" layer="94"/>
<wire x1="-20.32" y1="-33.02" x2="-20.32" y2="30.48" width="0.254" layer="94"/>
<pin name="VBAT" x="-25.4" y="27.94" length="middle"/>
<text x="-20.32" y="33.02" size="2.54" layer="95">&gt;NAME</text>
<pin name="PC13-TAMPER-RTC" x="-25.4" y="25.4" length="middle"/>
<pin name="PC14-OSC32_IN" x="-25.4" y="22.86" length="middle"/>
<pin name="PC15-OSC32_OUT" x="-25.4" y="20.32" length="middle"/>
<pin name="OSC_IN" x="-25.4" y="17.78" length="middle"/>
<pin name="OSC_OUT" x="-25.4" y="15.24" length="middle"/>
<pin name="NRST" x="-25.4" y="12.7" length="middle"/>
<pin name="VSSA" x="-25.4" y="10.16" length="middle"/>
<pin name="VDDA" x="-25.4" y="7.62" length="middle"/>
<pin name="PA0-WKUP" x="-25.4" y="5.08" length="middle"/>
<pin name="PA1" x="-25.4" y="2.54" length="middle"/>
<pin name="PA2" x="-25.4" y="0" length="middle"/>
<pin name="PA3" x="-25.4" y="-2.54" length="middle"/>
<pin name="PA5" x="-25.4" y="-7.62" length="middle"/>
<pin name="PA4" x="-25.4" y="-5.08" length="middle"/>
<pin name="PA6" x="-25.4" y="-10.16" length="middle"/>
<pin name="PA7" x="-25.4" y="-12.7" length="middle"/>
<pin name="PB0" x="-25.4" y="-15.24" length="middle"/>
<pin name="PB1" x="-25.4" y="-17.78" length="middle"/>
<pin name="PB2/BOOT1" x="-25.4" y="-20.32" length="middle"/>
<pin name="PB10" x="-25.4" y="-22.86" length="middle"/>
<pin name="PB11" x="-25.4" y="-25.4" length="middle"/>
<pin name="VSS_1" x="-25.4" y="-27.94" length="middle"/>
<pin name="VDD_1" x="-25.4" y="-30.48" length="middle"/>
<pin name="PB12" x="25.4" y="-30.48" length="middle" rot="R180"/>
<pin name="PB13" x="25.4" y="-27.94" length="middle" rot="R180"/>
<pin name="PB14" x="25.4" y="-25.4" length="middle" rot="R180"/>
<pin name="PB15" x="25.4" y="-22.86" length="middle" rot="R180"/>
<pin name="PA8" x="25.4" y="-20.32" length="middle" rot="R180"/>
<pin name="PA9" x="25.4" y="-17.78" length="middle" rot="R180"/>
<pin name="PA10" x="25.4" y="-15.24" length="middle" rot="R180"/>
<pin name="PA11" x="25.4" y="-12.7" length="middle" rot="R180"/>
<pin name="PA12" x="25.4" y="-10.16" length="middle" rot="R180"/>
<pin name="PA13/JTMS/SWDIO" x="25.4" y="-7.62" length="middle" rot="R180"/>
<pin name="VSS_2" x="25.4" y="-5.08" length="middle" rot="R180"/>
<pin name="VDD_2" x="25.4" y="-2.54" length="middle" rot="R180"/>
<pin name="PA14/JTCK/SWCLK" x="25.4" y="0" length="middle" rot="R180"/>
<pin name="PA15/JTDI" x="25.4" y="2.54" length="middle" rot="R180"/>
<pin name="PB3/JTDO" x="25.4" y="5.08" length="middle" rot="R180"/>
<pin name="PB4/JNTRST" x="25.4" y="7.62" length="middle" rot="R180"/>
<pin name="PB5" x="25.4" y="10.16" length="middle" rot="R180"/>
<pin name="PB6" x="25.4" y="12.7" length="middle" rot="R180"/>
<pin name="PB7" x="25.4" y="15.24" length="middle" rot="R180"/>
<pin name="BOOT0" x="25.4" y="17.78" length="middle" rot="R180"/>
<pin name="PB8" x="25.4" y="20.32" length="middle" rot="R180"/>
<pin name="PB9" x="25.4" y="22.86" length="middle" rot="R180"/>
<pin name="VSS_3" x="25.4" y="25.4" length="middle" rot="R180"/>
<pin name="VDD_3" x="25.4" y="27.94" length="middle" rot="R180"/>
<text x="-20.32" y="-38.1" size="2.54" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="+3V3">
<wire x1="1.27" y1="-1.905" x2="0" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="0" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-2.54" y="-5.08" size="1.778" layer="96" rot="R90">&gt;VALUE</text>
<pin name="+3V3" x="0" y="-2.54" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
<symbol name="GND">
<wire x1="-1.905" y1="0" x2="1.905" y2="0" width="0.254" layer="94"/>
<text x="-2.54" y="-2.54" size="1.778" layer="96">&gt;VALUE</text>
<pin name="GND" x="0" y="2.54" visible="off" length="short" direction="sup" rot="R270"/>
</symbol>
<symbol name="C">
<wire x1="0" y1="0" x2="0" y2="-0.508" width="0.1524" layer="94"/>
<wire x1="0" y1="-2.54" x2="0" y2="-2.032" width="0.1524" layer="94"/>
<text x="1.524" y="0.381" size="1.778" layer="95">&gt;NAME</text>
<text x="1.524" y="-4.699" size="1.778" layer="96">&gt;VALUE</text>
<rectangle x1="-2.032" y1="-2.032" x2="2.032" y2="-1.524" layer="94"/>
<rectangle x1="-2.032" y1="-1.016" x2="2.032" y2="-0.508" layer="94"/>
<pin name="1" x="0" y="2.54" visible="off" length="short" direction="pas" swaplevel="1" rot="R270"/>
<pin name="2" x="0" y="-5.08" visible="off" length="short" direction="pas" swaplevel="1" rot="R90"/>
</symbol>
<symbol name="PINH2X5">
<wire x1="-6.35" y1="-7.62" x2="8.89" y2="-7.62" width="0.4064" layer="94"/>
<wire x1="8.89" y1="-7.62" x2="8.89" y2="7.62" width="0.4064" layer="94"/>
<wire x1="8.89" y1="7.62" x2="-6.35" y2="7.62" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="7.62" x2="-6.35" y2="-7.62" width="0.4064" layer="94"/>
<text x="-6.35" y="8.255" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-10.16" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="5.08" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="5.08" y="5.08" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="3" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="4" x="5.08" y="2.54" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="5" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="6" x="5.08" y="0" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="7" x="-2.54" y="-2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="8" x="5.08" y="-2.54" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="9" x="-2.54" y="-5.08" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="10" x="5.08" y="-5.08" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
</symbol>
<symbol name="PINH2X3">
<wire x1="-6.35" y1="-5.08" x2="8.89" y2="-5.08" width="0.4064" layer="94"/>
<wire x1="8.89" y1="-5.08" x2="8.89" y2="5.08" width="0.4064" layer="94"/>
<wire x1="8.89" y1="5.08" x2="-6.35" y2="5.08" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="5.08" x2="-6.35" y2="-5.08" width="0.4064" layer="94"/>
<text x="-6.35" y="5.715" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-7.62" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="5.08" y="2.54" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="3" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="4" x="5.08" y="0" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
<pin name="5" x="-2.54" y="-2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="6" x="5.08" y="-2.54" visible="pad" length="short" direction="pas" function="dot" rot="R180"/>
</symbol>
<symbol name="PINHD5">
<wire x1="-6.35" y1="-7.62" x2="1.27" y2="-7.62" width="0.4064" layer="94"/>
<wire x1="1.27" y1="-7.62" x2="1.27" y2="7.62" width="0.4064" layer="94"/>
<wire x1="1.27" y1="7.62" x2="-6.35" y2="7.62" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="7.62" x2="-6.35" y2="-7.62" width="0.4064" layer="94"/>
<text x="-6.35" y="8.255" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-10.16" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="5.08" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="3" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="4" x="-2.54" y="-2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="5" x="-2.54" y="-5.08" visible="pad" length="short" direction="pas" function="dot"/>
</symbol>
<symbol name="Q">
<wire x1="1.016" y1="0" x2="2.54" y2="0" width="0.1524" layer="94"/>
<wire x1="-2.54" y1="0" x2="-1.016" y2="0" width="0.1524" layer="94"/>
<wire x1="-0.381" y1="1.524" x2="-0.381" y2="-1.524" width="0.254" layer="94"/>
<wire x1="-0.381" y1="-1.524" x2="0.381" y2="-1.524" width="0.254" layer="94"/>
<wire x1="0.381" y1="-1.524" x2="0.381" y2="1.524" width="0.254" layer="94"/>
<wire x1="0.381" y1="1.524" x2="-0.381" y2="1.524" width="0.254" layer="94"/>
<wire x1="1.016" y1="1.778" x2="1.016" y2="-1.778" width="0.254" layer="94"/>
<wire x1="-1.016" y1="1.778" x2="-1.016" y2="-1.778" width="0.254" layer="94"/>
<text x="2.54" y="1.016" size="1.778" layer="95">&gt;NAME</text>
<text x="2.54" y="-2.54" size="1.778" layer="96">&gt;VALUE</text>
<text x="-2.159" y="-1.143" size="0.8636" layer="93">1</text>
<text x="1.524" y="-1.143" size="0.8636" layer="93">2</text>
<pin name="2" x="2.54" y="0" visible="off" length="point" direction="pas" swaplevel="1" rot="R180"/>
<pin name="1" x="-2.54" y="0" visible="off" length="point" direction="pas" swaplevel="1"/>
</symbol>
<symbol name="R">
<wire x1="-2.54" y1="-0.889" x2="2.54" y2="-0.889" width="0.254" layer="94"/>
<wire x1="2.54" y1="0.889" x2="-2.54" y2="0.889" width="0.254" layer="94"/>
<wire x1="2.54" y1="-0.889" x2="2.54" y2="0.889" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-0.889" x2="-2.54" y2="0.889" width="0.254" layer="94"/>
<text x="-3.81" y="1.4986" size="1.778" layer="95">&gt;NAME</text>
<text x="-3.81" y="-3.302" size="1.778" layer="96">&gt;VALUE</text>
<pin name="2" x="5.08" y="0" visible="off" length="short" direction="pas" swaplevel="1" rot="R180"/>
<pin name="1" x="-5.08" y="0" visible="off" length="short" direction="pas" swaplevel="1"/>
</symbol>
<symbol name="SCHMITT">
<wire x1="-5.08" y1="5.08" x2="5.08" y2="0" width="0.4064" layer="94"/>
<wire x1="5.08" y1="0" x2="-5.08" y2="-5.08" width="0.4064" layer="94"/>
<wire x1="-5.08" y1="-5.08" x2="-5.08" y2="5.08" width="0.4064" layer="94"/>
<wire x1="-0.762" y1="-1.27" x2="-1.778" y2="1.27" width="0.1524" layer="94"/>
<wire x1="-2.032" y1="-1.27" x2="-3.048" y2="1.27" width="0.1524" layer="94"/>
<wire x1="-2.032" y1="-1.27" x2="-0.762" y2="-1.27" width="0.1524" layer="94"/>
<wire x1="-0.762" y1="-1.27" x2="0.127" y2="-1.27" width="0.1524" layer="94"/>
<wire x1="-3.937" y1="1.27" x2="-3.048" y2="1.27" width="0.1524" layer="94"/>
<wire x1="-3.048" y1="1.27" x2="-1.778" y2="1.27" width="0.1524" layer="94"/>
<text x="1.27" y="3.175" size="1.778" layer="95">&gt;NAME</text>
<text x="1.27" y="-5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="I" x="-10.16" y="0" visible="pad" length="middle" direction="in"/>
<pin name="O" x="10.16" y="0" visible="pad" length="middle" direction="out" function="dot" rot="R180"/>
</symbol>
<symbol name="PWRN">
<text x="-0.635" y="-0.635" size="1.778" layer="95">&gt;NAME</text>
<text x="1.905" y="-7.62" size="1.27" layer="95" rot="R90">GND</text>
<text x="1.905" y="5.08" size="1.27" layer="95" rot="R90">VCC</text>
<pin name="GND" x="0" y="-10.16" visible="pad" direction="pwr" rot="R90"/>
<pin name="VCC" x="0" y="10.16" visible="pad" direction="pwr" rot="R270"/>
</symbol>
<symbol name="PINHD4">
<wire x1="-6.35" y1="-5.08" x2="1.27" y2="-5.08" width="0.4064" layer="94"/>
<wire x1="1.27" y1="-5.08" x2="1.27" y2="7.62" width="0.4064" layer="94"/>
<wire x1="1.27" y1="7.62" x2="-6.35" y2="7.62" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="7.62" x2="-6.35" y2="-5.08" width="0.4064" layer="94"/>
<text x="-6.35" y="8.255" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-7.62" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="5.08" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="3" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="4" x="-2.54" y="-2.54" visible="pad" length="short" direction="pas" function="dot"/>
</symbol>
<symbol name="PUSH">
<pin name="1" x="-7.62" y="0" visible="off" length="middle"/>
<pin name="2" x="7.62" y="0" visible="off" length="middle" rot="R180"/>
<wire x1="-2.54" y1="0" x2="2.54" y2="2.54" width="0.254" layer="94"/>
<text x="-5.08" y="5.08" size="1.778" layer="95">&gt;NAME</text>
<text x="-5.08" y="-2.54" size="1.778" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="+5V">
<wire x1="1.27" y1="-1.905" x2="0" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="0" x2="-1.27" y2="-1.905" width="0.254" layer="94"/>
<text x="-2.54" y="-5.08" size="1.778" layer="96" rot="R90">&gt;VALUE</text>
<pin name="+5V" x="0" y="-2.54" visible="off" length="short" direction="sup" rot="R90"/>
</symbol>
<symbol name="PINHD2">
<wire x1="-6.35" y1="-2.54" x2="1.27" y2="-2.54" width="0.4064" layer="94"/>
<wire x1="1.27" y1="-2.54" x2="1.27" y2="5.08" width="0.4064" layer="94"/>
<wire x1="1.27" y1="5.08" x2="-6.35" y2="5.08" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="5.08" x2="-6.35" y2="-2.54" width="0.4064" layer="94"/>
<text x="-6.35" y="5.715" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-5.08" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
</symbol>
<symbol name="MOUNT-PAD">
<wire x1="0.254" y1="2.032" x2="2.032" y2="0.254" width="1.016" layer="94" curve="-75.749967" cap="flat"/>
<wire x1="-2.032" y1="0.254" x2="-0.254" y2="2.032" width="1.016" layer="94" curve="-75.749967" cap="flat"/>
<wire x1="-2.032" y1="-0.254" x2="-0.254" y2="-2.032" width="1.016" layer="94" curve="75.749967" cap="flat"/>
<wire x1="0.254" y1="-2.032" x2="2.032" y2="-0.254" width="1.016" layer="94" curve="75.749967" cap="flat"/>
<circle x="0" y="0" radius="1.524" width="0.0508" layer="94"/>
<text x="2.794" y="0.5842" size="1.778" layer="95">&gt;NAME</text>
<text x="2.794" y="-2.4638" size="1.778" layer="96">&gt;VALUE</text>
<pin name="MOUNT" x="-2.54" y="0" visible="off" length="short" direction="pas"/>
</symbol>
<symbol name="RT9193">
<wire x1="-7.62" y1="7.62" x2="-7.62" y2="-10.16" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-10.16" x2="10.16" y2="-10.16" width="0.254" layer="94"/>
<wire x1="10.16" y1="-10.16" x2="10.16" y2="7.62" width="0.254" layer="94"/>
<wire x1="10.16" y1="7.62" x2="-7.62" y2="7.62" width="0.254" layer="94"/>
<pin name="VIN" x="-12.7" y="5.08" length="middle" direction="sup"/>
<pin name="GND" x="-12.7" y="-7.62" length="middle" direction="pwr"/>
<pin name="EN" x="-12.7" y="0" length="middle"/>
<pin name="VOUT" x="15.24" y="5.08" length="middle" direction="sup" rot="R180"/>
<pin name="BP" x="15.24" y="-7.62" length="middle" rot="R180"/>
<text x="-7.62" y="10.16" size="2.54" layer="95">&gt;NAME</text>
<text x="-7.62" y="-15.24" size="2.54" layer="96">&gt;VALUE</text>
</symbol>
<symbol name="PINHD3">
<wire x1="-6.35" y1="-5.08" x2="1.27" y2="-5.08" width="0.4064" layer="94"/>
<wire x1="1.27" y1="-5.08" x2="1.27" y2="5.08" width="0.4064" layer="94"/>
<wire x1="1.27" y1="5.08" x2="-6.35" y2="5.08" width="0.4064" layer="94"/>
<wire x1="-6.35" y1="5.08" x2="-6.35" y2="-5.08" width="0.4064" layer="94"/>
<text x="-6.35" y="5.715" size="1.778" layer="95">&gt;NAME</text>
<text x="-6.35" y="-7.62" size="1.778" layer="96">&gt;VALUE</text>
<pin name="1" x="-2.54" y="2.54" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="2" x="-2.54" y="0" visible="pad" length="short" direction="pas" function="dot"/>
<pin name="3" x="-2.54" y="-2.54" visible="pad" length="short" direction="pas" function="dot"/>
</symbol>
<symbol name="R-TRIM">
<wire x1="0.762" y1="2.54" x2="0" y2="2.54" width="0.254" layer="94"/>
<wire x1="-0.762" y1="2.54" x2="-0.762" y2="-2.54" width="0.254" layer="94"/>
<wire x1="0.762" y1="-2.54" x2="0.762" y2="2.54" width="0.254" layer="94"/>
<wire x1="2.54" y1="0" x2="1.651" y2="0" width="0.1524" layer="94"/>
<wire x1="1.651" y1="0" x2="-1.8796" y2="1.7526" width="0.1524" layer="94"/>
<wire x1="0" y1="2.54" x2="0" y2="5.08" width="0.1524" layer="94"/>
<wire x1="0" y1="2.54" x2="-0.762" y2="2.54" width="0.254" layer="94"/>
<wire x1="-0.762" y1="-2.54" x2="0.762" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.286" y1="1.27" x2="-1.651" y2="2.413" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-2.54" x2="-2.54" y2="-0.508" width="0.1524" layer="94"/>
<wire x1="-2.54" y1="-0.508" x2="-3.048" y2="-1.524" width="0.1524" layer="94"/>
<wire x1="-2.54" y1="-0.508" x2="-2.032" y2="-1.524" width="0.1524" layer="94"/>
<text x="-5.969" y="-3.81" size="1.778" layer="95" rot="R90">&gt;NAME</text>
<text x="-3.81" y="-3.81" size="1.778" layer="96" rot="R90">&gt;VALUE</text>
<pin name="A" x="0" y="-5.08" visible="pad" length="short" direction="pas" rot="R90"/>
<pin name="E" x="0" y="5.08" visible="pad" length="short" direction="pas" rot="R270"/>
<pin name="S" x="5.08" y="0" visible="pad" length="short" direction="pas" rot="R180"/>
</symbol>
<symbol name="LED">
<wire x1="1.27" y1="0" x2="0" y2="-2.54" width="0.254" layer="94"/>
<wire x1="0" y1="-2.54" x2="-1.27" y2="0" width="0.254" layer="94"/>
<wire x1="1.27" y1="-2.54" x2="0" y2="-2.54" width="0.254" layer="94"/>
<wire x1="0" y1="-2.54" x2="-1.27" y2="-2.54" width="0.254" layer="94"/>
<wire x1="1.27" y1="0" x2="0" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="0" x2="-1.27" y2="0" width="0.254" layer="94"/>
<wire x1="0" y1="0" x2="0" y2="-2.54" width="0.1524" layer="94"/>
<wire x1="-2.032" y1="-0.762" x2="-3.429" y2="-2.159" width="0.1524" layer="94"/>
<wire x1="-1.905" y1="-1.905" x2="-3.302" y2="-3.302" width="0.1524" layer="94"/>
<text x="3.556" y="-4.572" size="1.778" layer="95" rot="R90">&gt;NAME</text>
<text x="5.715" y="-4.572" size="1.778" layer="96" rot="R90">&gt;VALUE</text>
<pin name="C" x="0" y="-5.08" visible="off" length="short" direction="pas" rot="R90"/>
<pin name="A" x="0" y="2.54" visible="off" length="short" direction="pas" rot="R270"/>
<polygon width="0.1524" layer="94">
<vertex x="-3.429" y="-2.159"/>
<vertex x="-3.048" y="-1.27"/>
<vertex x="-2.54" y="-1.778"/>
</polygon>
<polygon width="0.1524" layer="94">
<vertex x="-3.302" y="-3.302"/>
<vertex x="-2.921" y="-2.413"/>
<vertex x="-2.413" y="-2.921"/>
</polygon>
</symbol>
</symbols>
<devicesets>
<deviceset name="STM32F103C8XX" prefix="U">
<gates>
<gate name="G$1" symbol="STM32F103C8" x="0" y="0"/>
</gates>
<devices>
<device name="" package="LQFP48-7X7MM">
<connects>
<connect gate="G$1" pin="BOOT0" pad="44"/>
<connect gate="G$1" pin="NRST" pad="7"/>
<connect gate="G$1" pin="OSC_IN" pad="5"/>
<connect gate="G$1" pin="OSC_OUT" pad="6"/>
<connect gate="G$1" pin="PA0-WKUP" pad="10"/>
<connect gate="G$1" pin="PA1" pad="11"/>
<connect gate="G$1" pin="PA10" pad="31"/>
<connect gate="G$1" pin="PA11" pad="32"/>
<connect gate="G$1" pin="PA12" pad="33"/>
<connect gate="G$1" pin="PA13/JTMS/SWDIO" pad="34"/>
<connect gate="G$1" pin="PA14/JTCK/SWCLK" pad="37"/>
<connect gate="G$1" pin="PA15/JTDI" pad="38"/>
<connect gate="G$1" pin="PA2" pad="12"/>
<connect gate="G$1" pin="PA3" pad="13"/>
<connect gate="G$1" pin="PA4" pad="14"/>
<connect gate="G$1" pin="PA5" pad="15"/>
<connect gate="G$1" pin="PA6" pad="16"/>
<connect gate="G$1" pin="PA7" pad="17"/>
<connect gate="G$1" pin="PA8" pad="29"/>
<connect gate="G$1" pin="PA9" pad="30"/>
<connect gate="G$1" pin="PB0" pad="18"/>
<connect gate="G$1" pin="PB1" pad="19"/>
<connect gate="G$1" pin="PB10" pad="21"/>
<connect gate="G$1" pin="PB11" pad="22"/>
<connect gate="G$1" pin="PB12" pad="25"/>
<connect gate="G$1" pin="PB13" pad="26"/>
<connect gate="G$1" pin="PB14" pad="27"/>
<connect gate="G$1" pin="PB15" pad="28"/>
<connect gate="G$1" pin="PB2/BOOT1" pad="20"/>
<connect gate="G$1" pin="PB3/JTDO" pad="39"/>
<connect gate="G$1" pin="PB4/JNTRST" pad="40"/>
<connect gate="G$1" pin="PB5" pad="41"/>
<connect gate="G$1" pin="PB6" pad="42"/>
<connect gate="G$1" pin="PB7" pad="43"/>
<connect gate="G$1" pin="PB8" pad="45"/>
<connect gate="G$1" pin="PB9" pad="46"/>
<connect gate="G$1" pin="PC13-TAMPER-RTC" pad="2"/>
<connect gate="G$1" pin="PC14-OSC32_IN" pad="3"/>
<connect gate="G$1" pin="PC15-OSC32_OUT" pad="4"/>
<connect gate="G$1" pin="VBAT" pad="1"/>
<connect gate="G$1" pin="VDDA" pad="9"/>
<connect gate="G$1" pin="VDD_1" pad="24"/>
<connect gate="G$1" pin="VDD_2" pad="36"/>
<connect gate="G$1" pin="VDD_3" pad="48"/>
<connect gate="G$1" pin="VSSA" pad="8"/>
<connect gate="G$1" pin="VSS_1" pad="23"/>
<connect gate="G$1" pin="VSS_2" pad="35"/>
<connect gate="G$1" pin="VSS_3" pad="47"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="+3V3" prefix="+3V3">
<description>&lt;b&gt;SUPPLY SYMBOL&lt;/b&gt;</description>
<gates>
<gate name="G$1" symbol="+3V3" x="0" y="0"/>
</gates>
<devices>
<device name="">
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="GND" prefix="GND">
<description>&lt;b&gt;SUPPLY SYMBOL&lt;/b&gt;</description>
<gates>
<gate name="1" symbol="GND" x="0" y="0"/>
</gates>
<devices>
<device name="">
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="C" prefix="C" uservalue="yes">
<gates>
<gate name="G$1" symbol="C" x="0" y="0"/>
</gates>
<devices>
<device name="C1608" package="C1608">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="C2012" package="C2012">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="C1005" package="C1005">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PINHD-2X5" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="A" symbol="PINH2X5" x="0" y="0"/>
</gates>
<devices>
<device name="" package="2X05">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="10" pad="10"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
<connect gate="A" pin="6" pad="6"/>
<connect gate="A" pin="7" pad="7"/>
<connect gate="A" pin="8" pad="8"/>
<connect gate="A" pin="9" pad="9"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="/90" package="2X05/90">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="10" pad="10"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
<connect gate="A" pin="6" pad="6"/>
<connect gate="A" pin="7" pad="7"/>
<connect gate="A" pin="8" pad="8"/>
<connect gate="A" pin="9" pad="9"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PINHD-2X3" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="A" symbol="PINH2X3" x="0" y="0"/>
</gates>
<devices>
<device name="" package="2X03">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
<connect gate="A" pin="6" pad="6"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="/90" package="2X03/90">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
<connect gate="A" pin="6" pad="6"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PINHD-1X5" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="A" symbol="PINHD5" x="0" y="0"/>
</gates>
<devices>
<device name="" package="1X05">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="/90" package="1X05/90">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="5X2MM" package="1_05X2MM">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
<connect gate="A" pin="5" pad="5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="CRYSTAL" prefix="Q" uservalue="yes">
<gates>
<gate name="G$1" symbol="Q" x="0" y="0"/>
</gates>
<devices>
<device name="9C" package="HC-49US">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="9B" package="HC49/S">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="R" prefix="R" uservalue="yes">
<gates>
<gate name="G$1" symbol="R" x="0" y="0"/>
</gates>
<devices>
<device name="R1608" package="R1608">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="74LCX14MTC" prefix="U">
<gates>
<gate name="A" symbol="SCHMITT" x="0" y="0"/>
<gate name="B" symbol="SCHMITT" x="0" y="-15.24"/>
<gate name="C" symbol="SCHMITT" x="25.4" y="0"/>
<gate name="D" symbol="SCHMITT" x="25.4" y="-15.24"/>
<gate name="E" symbol="SCHMITT" x="50.8" y="0"/>
<gate name="F" symbol="SCHMITT" x="50.8" y="-15.24"/>
<gate name="P" symbol="PWRN" x="73.66" y="-7.62"/>
</gates>
<devices>
<device name="" package="TSSOP14">
<connects>
<connect gate="A" pin="I" pad="1"/>
<connect gate="A" pin="O" pad="2"/>
<connect gate="B" pin="I" pad="3"/>
<connect gate="B" pin="O" pad="4"/>
<connect gate="C" pin="I" pad="5"/>
<connect gate="C" pin="O" pad="6"/>
<connect gate="D" pin="I" pad="9"/>
<connect gate="D" pin="O" pad="8"/>
<connect gate="E" pin="I" pad="11"/>
<connect gate="E" pin="O" pad="10"/>
<connect gate="F" pin="I" pad="13"/>
<connect gate="F" pin="O" pad="12"/>
<connect gate="P" pin="GND" pad="7"/>
<connect gate="P" pin="VCC" pad="14"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PINHD-1X4" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="A" symbol="PINHD4" x="0" y="0"/>
</gates>
<devices>
<device name="" package="1X04">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="/90" package="1X04/90">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
<connect gate="A" pin="4" pad="4"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PUSH" prefix="SW" uservalue="yes">
<gates>
<gate name="G$1" symbol="PUSH" x="5.08" y="-2.54"/>
</gates>
<devices>
<device name="" package="PUSH">
<connects>
<connect gate="G$1" pin="1" pad="1 2"/>
<connect gate="G$1" pin="2" pad="3 4"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="+5V" prefix="P+">
<description>&lt;b&gt;SUPPLY SYMBOL&lt;/b&gt;</description>
<gates>
<gate name="1" symbol="+5V" x="0" y="0"/>
</gates>
<devices>
<device name="">
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="PINHD-1X2" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="G$1" symbol="PINHD2" x="0" y="0"/>
</gates>
<devices>
<device name="" package="1X02">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="/90" package="1X02/90">
<connects>
<connect gate="G$1" pin="1" pad="1"/>
<connect gate="G$1" pin="2" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="MOUNT-PAD-ROUND" prefix="H">
<description>&lt;b&gt;MOUNTING PAD&lt;/b&gt;, round</description>
<gates>
<gate name="G$1" symbol="MOUNT-PAD" x="0" y="0"/>
</gates>
<devices>
<device name="2.8" package="2,8-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B2,8"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="3.0" package="3,0-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B3,0"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="3.2" package="3,2-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B3,2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="3.3" package="3,3-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B3,3"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="3.6" package="3,6-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B3,6"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="4.1" package="4,1-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B4,1"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="4.3" package="4,3-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B4,3"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="4.5" package="4,5-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B4,5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="5.0" package="5,0-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
<device name="5.5" package="5,5-PAD">
<connects>
<connect gate="G$1" pin="MOUNT" pad="B5,5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="RT9193">
<gates>
<gate name="G$1" symbol="RT9193" x="0" y="0"/>
</gates>
<devices>
<device name="" package="SOT23-5L">
<connects>
<connect gate="G$1" pin="BP" pad="4"/>
<connect gate="G$1" pin="EN" pad="3"/>
<connect gate="G$1" pin="GND" pad="2"/>
<connect gate="G$1" pin="VIN" pad="1"/>
<connect gate="G$1" pin="VOUT" pad="5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="5-103361-1" prefix="JP" uservalue="yes">
<description>&lt;b&gt;PIN HEADER&lt;/b&gt;</description>
<gates>
<gate name="A" symbol="PINHD3" x="0" y="0"/>
</gates>
<devices>
<device name="" package="5-103361-1">
<connects>
<connect gate="A" pin="1" pad="1"/>
<connect gate="A" pin="2" pad="2"/>
<connect gate="A" pin="3" pad="3"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="R-TRIMM" prefix="R" uservalue="yes">
<description>&lt;b&gt;Trimm resistor&lt;/b&gt;</description>
<gates>
<gate name="G$1" symbol="R-TRIM" x="0" y="0"/>
</gates>
<devices>
<device name="T93YA" package="RTRIMT93YA">
<connects>
<connect gate="G$1" pin="A" pad="3"/>
<connect gate="G$1" pin="E" pad="1"/>
<connect gate="G$1" pin="S" pad="2"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
<deviceset name="LED" prefix="D">
<gates>
<gate name="G$1" symbol="LED" x="0" y="2.54"/>
</gates>
<devices>
<device name="" package="LED2012">
<connects>
<connect gate="G$1" pin="A" pad="A"/>
<connect gate="G$1" pin="C" pad="C"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
</devicesets>
</library>
</libraries>
<attributes>
</attributes>
<variantdefs>
</variantdefs>
<classes>
<class number="0" name="default" width="0" drill="0">
</class>
</classes>
<parts>
<part name="U1" library="x2-feed" deviceset="STM32F103C8XX" device=""/>
<part name="+3V1" library="x2-feed" deviceset="+3V3" device=""/>
<part name="GND1" library="x2-feed" deviceset="GND" device=""/>
<part name="C3" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="C4" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="C5" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="C6" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="C1" library="x2-feed" deviceset="C" device="C2012" value="10uF"/>
<part name="LCD" library="x2-feed" deviceset="PINHD-2X5" device="" value="LCD"/>
<part name="DRV/PWR" library="x2-feed" deviceset="PINHD-2X3" device="" value="DRV"/>
<part name="C2" library="x2-feed" deviceset="C" device="C1608" value="1uF"/>
<part name="C14" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="GND2" library="x2-feed" deviceset="GND" device=""/>
<part name="STLINK" library="x2-feed" deviceset="PINHD-1X5" device="" value="SWD"/>
<part name="GND3" library="x2-feed" deviceset="GND" device=""/>
<part name="+3V2" library="x2-feed" deviceset="+3V3" device=""/>
<part name="Q1" library="x2-feed" deviceset="CRYSTAL" device="9B" value="8MHz"/>
<part name="C13" library="x2-feed" deviceset="C" device="C1608" value="26pF"/>
<part name="C12" library="x2-feed" deviceset="C" device="C1608" value="26pF"/>
<part name="GND4" library="x2-feed" deviceset="GND" device=""/>
<part name="R1" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R2" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="GND5" library="x2-feed" deviceset="GND" device=""/>
<part name="GND6" library="x2-feed" deviceset="GND" device=""/>
<part name="R3" library="x2-feed" deviceset="R" device="R1608" value="0"/>
<part name="C101" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R102" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R101" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="U2" library="x2-feed" deviceset="74LCX14MTC" device=""/>
<part name="C102" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R104" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R103" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="C103" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R106" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R105" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="C104" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R108" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R107" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="C105" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R110" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="R109" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="GND11" library="x2-feed" deviceset="GND" device=""/>
<part name="+3V4" library="x2-feed" deviceset="+3V3" device=""/>
<part name="CTRLS" library="x2-feed" deviceset="PINHD-1X4" device="" value="CTRLS"/>
<part name="ENCODER" library="x2-feed" deviceset="PINHD-1X4" device="" value="ENCODER"/>
<part name="GND7" library="x2-feed" deviceset="GND" device=""/>
<part name="GND8" library="x2-feed" deviceset="GND" device=""/>
<part name="GND9" library="x2-feed" deviceset="GND" device=""/>
<part name="+3V3" library="x2-feed" deviceset="+3V3" device=""/>
<part name="GND10" library="x2-feed" deviceset="GND" device=""/>
<part name="SW1" library="x2-feed" deviceset="PUSH" device="" value="RESET"/>
<part name="P+1" library="x2-feed" deviceset="+5V" device=""/>
<part name="GND12" library="x2-feed" deviceset="GND" device=""/>
<part name="C7" library="x2-feed" deviceset="C" device="C2012" value="10uF"/>
<part name="GND13" library="x2-feed" deviceset="GND" device=""/>
<part name="P+2" library="x2-feed" deviceset="+5V" device=""/>
<part name="R112" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="C106" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R111" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="ESTOP" library="x2-feed" deviceset="PINHD-1X2" device="" value="ESTOP"/>
<part name="GND14" library="x2-feed" deviceset="GND" device=""/>
<part name="H1" library="x2-feed" deviceset="MOUNT-PAD-ROUND" device="2.8"/>
<part name="H2" library="x2-feed" deviceset="MOUNT-PAD-ROUND" device="2.8"/>
<part name="H3" library="x2-feed" deviceset="MOUNT-PAD-ROUND" device="2.8"/>
<part name="H4" library="x2-feed" deviceset="MOUNT-PAD-ROUND" device="2.8"/>
<part name="R4" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="+3V5" library="x2-feed" deviceset="+3V3" device=""/>
<part name="C8" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="U3" library="x2-feed" deviceset="RT9193" device=""/>
<part name="C9" library="x2-feed" deviceset="C" device="C2012" value="10uF"/>
<part name="C10" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="+3V6" library="x2-feed" deviceset="+3V3" device=""/>
<part name="GND15" library="x2-feed" deviceset="GND" device=""/>
<part name="C11" library="x2-feed" deviceset="C" device="C1608" value="0.022uF"/>
<part name="HALL" library="x2-feed" deviceset="5-103361-1" device="" value="HALL"/>
<part name="GND16" library="x2-feed" deviceset="GND" device=""/>
<part name="+3V7" library="x2-feed" deviceset="+3V3" device=""/>
<part name="C15" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R5" library="x2-feed" deviceset="R" device="R1608" value="10k"/>
<part name="P+3" library="x2-feed" deviceset="+5V" device=""/>
<part name="R6" library="x2-feed" deviceset="R-TRIMM" device="T93YA" value="10k"/>
<part name="GND17" library="x2-feed" deviceset="GND" device=""/>
<part name="C16" library="x2-feed" deviceset="C" device="C1608" value="0.1uF"/>
<part name="R7" library="x2-feed" deviceset="R" device="R1608" value="220"/>
<part name="+3V8" library="x2-feed" deviceset="+3V3" device=""/>
<part name="D1" library="x2-feed" deviceset="LED" device=""/>
<part name="D2" library="x2-feed" deviceset="LED" device=""/>
<part name="R8" library="x2-feed" deviceset="R" device="R1608" value="220"/>
</parts>
<sheets>
<sheet>
<plain>
</plain>
<instances>
<instance part="U1" gate="G$1" x="195.58" y="96.52"/>
<instance part="+3V1" gate="G$1" x="297.18" y="144.78"/>
<instance part="GND1" gate="1" x="297.18" y="121.92"/>
<instance part="C3" gate="G$1" x="297.18" y="134.62"/>
<instance part="C4" gate="G$1" x="307.34" y="134.62"/>
<instance part="C5" gate="G$1" x="317.5" y="134.62"/>
<instance part="C6" gate="G$1" x="327.66" y="134.62"/>
<instance part="C1" gate="G$1" x="276.86" y="134.62"/>
<instance part="LCD" gate="A" x="279.4" y="38.1"/>
<instance part="DRV/PWR" gate="A" x="279.4" y="68.58"/>
<instance part="C2" gate="G$1" x="287.02" y="134.62"/>
<instance part="C14" gate="G$1" x="144.78" y="104.14"/>
<instance part="GND2" gate="1" x="144.78" y="83.82"/>
<instance part="STLINK" gate="A" x="279.4" y="109.22"/>
<instance part="GND3" gate="1" x="261.62" y="96.52"/>
<instance part="+3V2" gate="G$1" x="261.62" y="121.92"/>
<instance part="Q1" gate="G$1" x="134.62" y="152.4" rot="R180"/>
<instance part="C13" gate="G$1" x="142.24" y="144.78" rot="R180"/>
<instance part="C12" gate="G$1" x="127" y="144.78" rot="R180"/>
<instance part="GND4" gate="1" x="134.62" y="134.62"/>
<instance part="R1" gate="G$1" x="147.32" y="76.2"/>
<instance part="R2" gate="G$1" x="231.14" y="114.3"/>
<instance part="GND5" gate="1" x="139.7" y="68.58"/>
<instance part="GND6" gate="1" x="241.3" y="109.22"/>
<instance part="R3" gate="G$1" x="142.24" y="160.02" rot="R90"/>
<instance part="C101" gate="G$1" x="55.88" y="129.54"/>
<instance part="R102" gate="G$1" x="48.26" y="134.62"/>
<instance part="R101" gate="G$1" x="35.56" y="142.24" rot="R90"/>
<instance part="U2" gate="A" x="78.74" y="134.62"/>
<instance part="U2" gate="B" x="78.74" y="109.22"/>
<instance part="U2" gate="C" x="78.74" y="86.36"/>
<instance part="U2" gate="D" x="78.74" y="63.5"/>
<instance part="U2" gate="E" x="78.74" y="40.64"/>
<instance part="U2" gate="F" x="78.74" y="15.24"/>
<instance part="U2" gate="P" x="5.08" y="20.32"/>
<instance part="C102" gate="G$1" x="55.88" y="104.14"/>
<instance part="R104" gate="G$1" x="48.26" y="109.22"/>
<instance part="R103" gate="G$1" x="35.56" y="116.84" rot="R90"/>
<instance part="C103" gate="G$1" x="55.88" y="81.28"/>
<instance part="R106" gate="G$1" x="48.26" y="86.36"/>
<instance part="R105" gate="G$1" x="35.56" y="93.98" rot="R90"/>
<instance part="C104" gate="G$1" x="55.88" y="58.42"/>
<instance part="R108" gate="G$1" x="48.26" y="63.5"/>
<instance part="R107" gate="G$1" x="35.56" y="71.12" rot="R90"/>
<instance part="C105" gate="G$1" x="55.88" y="35.56"/>
<instance part="R110" gate="G$1" x="48.26" y="40.64"/>
<instance part="R109" gate="G$1" x="35.56" y="48.26" rot="R90"/>
<instance part="GND11" gate="1" x="66.04" y="-2.54"/>
<instance part="+3V4" gate="G$1" x="40.64" y="154.94"/>
<instance part="CTRLS" gate="A" x="5.08" y="86.36" rot="MR0"/>
<instance part="ENCODER" gate="A" x="5.08" y="53.34" rot="MR0"/>
<instance part="GND7" gate="1" x="22.86" y="30.48"/>
<instance part="GND8" gate="1" x="25.4" y="76.2"/>
<instance part="GND9" gate="1" x="5.08" y="2.54"/>
<instance part="+3V3" gate="G$1" x="5.08" y="38.1"/>
<instance part="GND10" gate="1" x="261.62" y="22.86"/>
<instance part="SW1" gate="G$1" x="137.16" y="99.06" rot="R90"/>
<instance part="P+1" gate="1" x="200.66" y="182.88"/>
<instance part="GND12" gate="1" x="200.66" y="144.78"/>
<instance part="C7" gate="G$1" x="200.66" y="172.72"/>
<instance part="GND13" gate="1" x="294.64" y="53.34"/>
<instance part="P+2" gate="1" x="264.16" y="83.82"/>
<instance part="R112" gate="G$1" x="48.26" y="15.24"/>
<instance part="C106" gate="G$1" x="55.88" y="10.16"/>
<instance part="R111" gate="G$1" x="35.56" y="22.86" rot="R90"/>
<instance part="ESTOP" gate="G$1" x="106.68" y="78.74" rot="MR0"/>
<instance part="GND14" gate="1" x="116.84" y="68.58"/>
<instance part="H1" gate="G$1" x="271.78" y="15.24" smashed="yes" rot="R180"/>
<instance part="H2" gate="G$1" x="271.78" y="7.62" smashed="yes" rot="R180"/>
<instance part="H3" gate="G$1" x="281.94" y="15.24" smashed="yes"/>
<instance part="H4" gate="G$1" x="281.94" y="7.62" smashed="yes"/>
<instance part="R4" gate="G$1" x="124.46" y="88.9" rot="R90"/>
<instance part="+3V5" gate="G$1" x="124.46" y="99.06"/>
<instance part="C8" gate="G$1" x="210.82" y="172.72"/>
<instance part="U3" gate="G$1" x="238.76" y="172.72"/>
<instance part="C9" gate="G$1" x="266.7" y="167.64"/>
<instance part="C10" gate="G$1" x="276.86" y="167.64"/>
<instance part="+3V6" gate="G$1" x="276.86" y="185.42"/>
<instance part="GND15" gate="1" x="276.86" y="147.32"/>
<instance part="C11" gate="G$1" x="256.54" y="160.02"/>
<instance part="HALL" gate="A" x="132.08" y="15.24" rot="MR0"/>
<instance part="GND16" gate="1" x="147.32" y="7.62"/>
<instance part="+3V7" gate="G$1" x="147.32" y="33.02"/>
<instance part="C15" gate="G$1" x="152.4" y="20.32"/>
<instance part="R5" gate="G$1" x="165.1" y="20.32" rot="R90"/>
<instance part="P+3" gate="1" x="261.62" y="50.8"/>
<instance part="R6" gate="G$1" x="246.38" y="35.56"/>
<instance part="GND17" gate="1" x="276.86" y="2.54"/>
<instance part="C16" gate="G$1" x="12.7" y="20.32"/>
<instance part="R7" gate="G$1" x="195.58" y="38.1" rot="R90"/>
<instance part="+3V8" gate="G$1" x="195.58" y="48.26"/>
<instance part="D1" gate="G$1" x="195.58" y="27.94"/>
<instance part="D2" gate="G$1" x="289.56" y="160.02"/>
<instance part="R8" gate="G$1" x="289.56" y="170.18" rot="R90"/>
</instances>
<busses>
</busses>
<nets>
<net name="GND" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="VSSA"/>
<wire x1="170.18" y1="106.68" x2="162.56" y2="106.68" width="0.1524" layer="91"/>
<label x="162.56" y="106.68" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VSS_1"/>
<wire x1="170.18" y1="68.58" x2="162.56" y2="68.58" width="0.1524" layer="91"/>
<label x="162.56" y="68.58" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VSS_3"/>
<wire x1="220.98" y1="121.92" x2="228.6" y2="121.92" width="0.1524" layer="91"/>
<label x="223.52" y="121.92" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VSS_2"/>
<wire x1="220.98" y1="91.44" x2="228.6" y2="91.44" width="0.1524" layer="91"/>
<label x="223.52" y="91.44" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="C3" gate="G$1" pin="2"/>
<pinref part="GND1" gate="1" pin="GND"/>
<wire x1="297.18" y1="129.54" x2="297.18" y2="127" width="0.1524" layer="91"/>
<pinref part="C4" gate="G$1" pin="2"/>
<wire x1="297.18" y1="127" x2="297.18" y2="124.46" width="0.1524" layer="91"/>
<wire x1="297.18" y1="127" x2="307.34" y2="127" width="0.1524" layer="91"/>
<wire x1="307.34" y1="127" x2="307.34" y2="129.54" width="0.1524" layer="91"/>
<junction x="297.18" y="127"/>
<junction x="307.34" y="127"/>
<pinref part="C5" gate="G$1" pin="2"/>
<wire x1="307.34" y1="127" x2="317.5" y2="127" width="0.1524" layer="91"/>
<wire x1="317.5" y1="127" x2="317.5" y2="129.54" width="0.1524" layer="91"/>
<pinref part="C6" gate="G$1" pin="2"/>
<wire x1="317.5" y1="127" x2="327.66" y2="127" width="0.1524" layer="91"/>
<wire x1="327.66" y1="127" x2="327.66" y2="129.54" width="0.1524" layer="91"/>
<junction x="317.5" y="127"/>
<pinref part="C1" gate="G$1" pin="2"/>
<wire x1="297.18" y1="127" x2="287.02" y2="127" width="0.1524" layer="91"/>
<wire x1="287.02" y1="127" x2="276.86" y2="127" width="0.1524" layer="91"/>
<wire x1="276.86" y1="127" x2="276.86" y2="129.54" width="0.1524" layer="91"/>
<pinref part="C2" gate="G$1" pin="2"/>
<wire x1="287.02" y1="129.54" x2="287.02" y2="127" width="0.1524" layer="91"/>
<junction x="287.02" y="127"/>
</segment>
<segment>
<pinref part="C14" gate="G$1" pin="2"/>
<pinref part="GND2" gate="1" pin="GND"/>
<wire x1="144.78" y1="99.06" x2="144.78" y2="88.9" width="0.1524" layer="91"/>
<pinref part="SW1" gate="G$1" pin="1"/>
<wire x1="144.78" y1="88.9" x2="144.78" y2="86.36" width="0.1524" layer="91"/>
<wire x1="137.16" y1="91.44" x2="137.16" y2="88.9" width="0.1524" layer="91"/>
<wire x1="137.16" y1="88.9" x2="144.78" y2="88.9" width="0.1524" layer="91"/>
<junction x="144.78" y="88.9"/>
</segment>
<segment>
<pinref part="STLINK" gate="A" pin="3"/>
<wire x1="276.86" y1="109.22" x2="261.62" y2="109.22" width="0.1524" layer="91"/>
<wire x1="261.62" y1="109.22" x2="261.62" y2="99.06" width="0.1524" layer="91"/>
<pinref part="GND3" gate="1" pin="GND"/>
<label x="264.16" y="109.22" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="C12" gate="G$1" pin="1"/>
<wire x1="127" y1="142.24" x2="127" y2="139.7" width="0.1524" layer="91"/>
<pinref part="C13" gate="G$1" pin="1"/>
<wire x1="142.24" y1="142.24" x2="142.24" y2="139.7" width="0.1524" layer="91"/>
<wire x1="127" y1="139.7" x2="134.62" y2="139.7" width="0.1524" layer="91"/>
<pinref part="GND4" gate="1" pin="GND"/>
<wire x1="134.62" y1="139.7" x2="142.24" y2="139.7" width="0.1524" layer="91"/>
<wire x1="134.62" y1="137.16" x2="134.62" y2="139.7" width="0.1524" layer="91"/>
<junction x="134.62" y="139.7"/>
</segment>
<segment>
<pinref part="R1" gate="G$1" pin="1"/>
<pinref part="GND5" gate="1" pin="GND"/>
<wire x1="142.24" y1="76.2" x2="139.7" y2="76.2" width="0.1524" layer="91"/>
<wire x1="139.7" y1="76.2" x2="139.7" y2="71.12" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="R2" gate="G$1" pin="2"/>
<pinref part="GND6" gate="1" pin="GND"/>
<wire x1="236.22" y1="114.3" x2="241.3" y2="114.3" width="0.1524" layer="91"/>
<wire x1="241.3" y1="114.3" x2="241.3" y2="111.76" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="C105" gate="G$1" pin="2"/>
<pinref part="GND11" gate="1" pin="GND"/>
<wire x1="55.88" y1="30.48" x2="55.88" y2="27.94" width="0.1524" layer="91"/>
<wire x1="55.88" y1="27.94" x2="66.04" y2="27.94" width="0.1524" layer="91"/>
<pinref part="C101" gate="G$1" pin="2"/>
<wire x1="55.88" y1="124.46" x2="55.88" y2="121.92" width="0.1524" layer="91"/>
<wire x1="55.88" y1="121.92" x2="66.04" y2="121.92" width="0.1524" layer="91"/>
<wire x1="66.04" y1="121.92" x2="66.04" y2="96.52" width="0.1524" layer="91"/>
<wire x1="66.04" y1="96.52" x2="66.04" y2="73.66" width="0.1524" layer="91"/>
<wire x1="66.04" y1="73.66" x2="66.04" y2="50.8" width="0.1524" layer="91"/>
<wire x1="66.04" y1="50.8" x2="66.04" y2="27.94" width="0.1524" layer="91"/>
<wire x1="66.04" y1="0" x2="66.04" y2="2.54" width="0.1524" layer="91"/>
<junction x="66.04" y="27.94"/>
<pinref part="C104" gate="G$1" pin="2"/>
<wire x1="66.04" y1="2.54" x2="66.04" y2="27.94" width="0.1524" layer="91"/>
<wire x1="55.88" y1="53.34" x2="55.88" y2="50.8" width="0.1524" layer="91"/>
<wire x1="55.88" y1="50.8" x2="66.04" y2="50.8" width="0.1524" layer="91"/>
<junction x="66.04" y="50.8"/>
<pinref part="C103" gate="G$1" pin="2"/>
<wire x1="55.88" y1="76.2" x2="55.88" y2="73.66" width="0.1524" layer="91"/>
<wire x1="55.88" y1="73.66" x2="66.04" y2="73.66" width="0.1524" layer="91"/>
<junction x="66.04" y="73.66"/>
<pinref part="C102" gate="G$1" pin="2"/>
<wire x1="55.88" y1="99.06" x2="55.88" y2="96.52" width="0.1524" layer="91"/>
<wire x1="55.88" y1="96.52" x2="66.04" y2="96.52" width="0.1524" layer="91"/>
<junction x="66.04" y="96.52"/>
<pinref part="C106" gate="G$1" pin="2"/>
<wire x1="55.88" y1="5.08" x2="55.88" y2="2.54" width="0.1524" layer="91"/>
<wire x1="55.88" y1="2.54" x2="66.04" y2="2.54" width="0.1524" layer="91"/>
<junction x="66.04" y="2.54"/>
</segment>
<segment>
<pinref part="CTRLS" gate="A" pin="4"/>
<pinref part="GND8" gate="1" pin="GND"/>
<wire x1="7.62" y1="83.82" x2="25.4" y2="83.82" width="0.1524" layer="91"/>
<wire x1="25.4" y1="83.82" x2="25.4" y2="78.74" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="U2" gate="P" pin="GND"/>
<pinref part="GND9" gate="1" pin="GND"/>
<wire x1="5.08" y1="10.16" x2="5.08" y2="7.62" width="0.1524" layer="91"/>
<pinref part="C16" gate="G$1" pin="2"/>
<wire x1="5.08" y1="7.62" x2="5.08" y2="5.08" width="0.1524" layer="91"/>
<wire x1="5.08" y1="7.62" x2="12.7" y2="7.62" width="0.1524" layer="91"/>
<wire x1="12.7" y1="7.62" x2="12.7" y2="15.24" width="0.1524" layer="91"/>
<junction x="5.08" y="7.62"/>
</segment>
<segment>
<pinref part="C7" gate="G$1" pin="2"/>
<pinref part="GND12" gate="1" pin="GND"/>
<wire x1="200.66" y1="167.64" x2="200.66" y2="165.1" width="0.1524" layer="91"/>
<pinref part="C8" gate="G$1" pin="2"/>
<wire x1="200.66" y1="165.1" x2="200.66" y2="147.32" width="0.1524" layer="91"/>
<wire x1="210.82" y1="167.64" x2="210.82" y2="165.1" width="0.1524" layer="91"/>
<wire x1="210.82" y1="165.1" x2="200.66" y2="165.1" width="0.1524" layer="91"/>
<junction x="200.66" y="165.1"/>
<pinref part="U3" gate="G$1" pin="GND"/>
<wire x1="226.06" y1="165.1" x2="210.82" y2="165.1" width="0.1524" layer="91"/>
<junction x="210.82" y="165.1"/>
</segment>
<segment>
<wire x1="294.64" y1="71.12" x2="294.64" y2="55.88" width="0.1524" layer="91"/>
<pinref part="GND13" gate="1" pin="GND"/>
<pinref part="DRV/PWR" gate="A" pin="2"/>
<wire x1="284.48" y1="71.12" x2="294.64" y2="71.12" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="ENCODER" gate="A" pin="4"/>
<pinref part="GND7" gate="1" pin="GND"/>
<wire x1="7.62" y1="50.8" x2="22.86" y2="50.8" width="0.1524" layer="91"/>
<wire x1="22.86" y1="50.8" x2="22.86" y2="33.02" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="ESTOP" gate="G$1" pin="2"/>
<wire x1="109.22" y1="78.74" x2="116.84" y2="78.74" width="0.1524" layer="91"/>
<wire x1="116.84" y1="78.74" x2="116.84" y2="71.12" width="0.1524" layer="91"/>
<pinref part="GND14" gate="1" pin="GND"/>
</segment>
<segment>
<pinref part="C9" gate="G$1" pin="2"/>
<pinref part="C10" gate="G$1" pin="2"/>
<pinref part="GND15" gate="1" pin="GND"/>
<wire x1="276.86" y1="162.56" x2="276.86" y2="152.4" width="0.1524" layer="91"/>
<pinref part="C11" gate="G$1" pin="2"/>
<wire x1="276.86" y1="152.4" x2="276.86" y2="149.86" width="0.1524" layer="91"/>
<wire x1="256.54" y1="154.94" x2="256.54" y2="152.4" width="0.1524" layer="91"/>
<wire x1="256.54" y1="152.4" x2="266.7" y2="152.4" width="0.1524" layer="91"/>
<junction x="276.86" y="152.4"/>
<wire x1="266.7" y1="152.4" x2="276.86" y2="152.4" width="0.1524" layer="91"/>
<wire x1="289.56" y1="154.94" x2="289.56" y2="152.4" width="0.1524" layer="91"/>
<wire x1="289.56" y1="152.4" x2="276.86" y2="152.4" width="0.1524" layer="91"/>
<pinref part="D2" gate="G$1" pin="C"/>
<wire x1="266.7" y1="162.56" x2="266.7" y2="152.4" width="0.1524" layer="91"/>
<junction x="266.7" y="152.4"/>
</segment>
<segment>
<pinref part="HALL" gate="A" pin="2"/>
<wire x1="134.62" y1="15.24" x2="147.32" y2="15.24" width="0.1524" layer="91"/>
<wire x1="147.32" y1="15.24" x2="147.32" y2="10.16" width="0.1524" layer="91"/>
<pinref part="GND16" gate="1" pin="GND"/>
<pinref part="C15" gate="G$1" pin="2"/>
<wire x1="147.32" y1="15.24" x2="152.4" y2="15.24" width="0.1524" layer="91"/>
<junction x="147.32" y="15.24"/>
</segment>
<segment>
<pinref part="LCD" gate="A" pin="9"/>
<wire x1="276.86" y1="33.02" x2="261.62" y2="33.02" width="0.1524" layer="91"/>
<pinref part="GND10" gate="1" pin="GND"/>
<wire x1="261.62" y1="33.02" x2="261.62" y2="27.94" width="0.1524" layer="91"/>
<pinref part="R6" gate="G$1" pin="A"/>
<wire x1="261.62" y1="27.94" x2="261.62" y2="25.4" width="0.1524" layer="91"/>
<wire x1="246.38" y1="30.48" x2="246.38" y2="27.94" width="0.1524" layer="91"/>
<wire x1="246.38" y1="27.94" x2="261.62" y2="27.94" width="0.1524" layer="91"/>
<junction x="261.62" y="27.94"/>
</segment>
<segment>
<pinref part="H1" gate="G$1" pin="MOUNT"/>
<pinref part="H3" gate="G$1" pin="MOUNT"/>
<wire x1="274.32" y1="15.24" x2="276.86" y2="15.24" width="0.1524" layer="91"/>
<wire x1="276.86" y1="15.24" x2="279.4" y2="15.24" width="0.1524" layer="91"/>
<wire x1="276.86" y1="15.24" x2="276.86" y2="7.62" width="0.1524" layer="91"/>
<junction x="276.86" y="15.24"/>
<pinref part="H2" gate="G$1" pin="MOUNT"/>
<wire x1="276.86" y1="7.62" x2="274.32" y2="7.62" width="0.1524" layer="91"/>
<pinref part="H4" gate="G$1" pin="MOUNT"/>
<wire x1="276.86" y1="7.62" x2="279.4" y2="7.62" width="0.1524" layer="91"/>
<junction x="276.86" y="7.62"/>
<pinref part="GND17" gate="1" pin="GND"/>
<wire x1="276.86" y1="7.62" x2="276.86" y2="5.08" width="0.1524" layer="91"/>
</segment>
</net>
<net name="+3V3" class="0">
<segment>
<pinref part="+3V1" gate="G$1" pin="+3V3"/>
<pinref part="C3" gate="G$1" pin="1"/>
<wire x1="297.18" y1="142.24" x2="297.18" y2="139.7" width="0.1524" layer="91"/>
<pinref part="C4" gate="G$1" pin="1"/>
<wire x1="297.18" y1="139.7" x2="297.18" y2="137.16" width="0.1524" layer="91"/>
<wire x1="297.18" y1="139.7" x2="307.34" y2="139.7" width="0.1524" layer="91"/>
<wire x1="307.34" y1="139.7" x2="307.34" y2="137.16" width="0.1524" layer="91"/>
<junction x="297.18" y="139.7"/>
<junction x="307.34" y="139.7"/>
<pinref part="C5" gate="G$1" pin="1"/>
<wire x1="307.34" y1="139.7" x2="317.5" y2="139.7" width="0.1524" layer="91"/>
<wire x1="317.5" y1="139.7" x2="317.5" y2="137.16" width="0.1524" layer="91"/>
<pinref part="C6" gate="G$1" pin="1"/>
<wire x1="317.5" y1="139.7" x2="327.66" y2="139.7" width="0.1524" layer="91"/>
<wire x1="327.66" y1="139.7" x2="327.66" y2="137.16" width="0.1524" layer="91"/>
<junction x="317.5" y="139.7"/>
<pinref part="C1" gate="G$1" pin="1"/>
<wire x1="297.18" y1="139.7" x2="287.02" y2="139.7" width="0.1524" layer="91"/>
<wire x1="287.02" y1="139.7" x2="276.86" y2="139.7" width="0.1524" layer="91"/>
<wire x1="276.86" y1="139.7" x2="276.86" y2="137.16" width="0.1524" layer="91"/>
<pinref part="C2" gate="G$1" pin="1"/>
<wire x1="287.02" y1="137.16" x2="287.02" y2="139.7" width="0.1524" layer="91"/>
<junction x="287.02" y="139.7"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VDD_3"/>
<wire x1="220.98" y1="124.46" x2="228.6" y2="124.46" width="0.1524" layer="91"/>
<label x="223.52" y="124.46" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VDD_2"/>
<wire x1="220.98" y1="93.98" x2="228.6" y2="93.98" width="0.1524" layer="91"/>
<label x="223.52" y="93.98" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VDDA"/>
<wire x1="170.18" y1="104.14" x2="162.56" y2="104.14" width="0.1524" layer="91"/>
<label x="162.56" y="104.14" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VDD_1"/>
<wire x1="170.18" y1="66.04" x2="162.56" y2="66.04" width="0.1524" layer="91"/>
<label x="162.56" y="66.04" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="VBAT"/>
<wire x1="170.18" y1="124.46" x2="162.56" y2="124.46" width="0.1524" layer="91"/>
<label x="162.56" y="124.46" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="STLINK" gate="A" pin="1"/>
<pinref part="+3V2" gate="G$1" pin="+3V3"/>
<wire x1="276.86" y1="114.3" x2="261.62" y2="114.3" width="0.1524" layer="91"/>
<wire x1="261.62" y1="114.3" x2="261.62" y2="119.38" width="0.1524" layer="91"/>
<label x="264.16" y="114.3" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="R109" gate="G$1" pin="2"/>
<wire x1="35.56" y1="55.88" x2="35.56" y2="53.34" width="0.1524" layer="91"/>
<wire x1="35.56" y1="55.88" x2="40.64" y2="55.88" width="0.1524" layer="91"/>
<wire x1="40.64" y1="55.88" x2="40.64" y2="78.74" width="0.1524" layer="91"/>
<pinref part="+3V4" gate="G$1" pin="+3V3"/>
<pinref part="R107" gate="G$1" pin="2"/>
<wire x1="40.64" y1="78.74" x2="40.64" y2="101.6" width="0.1524" layer="91"/>
<wire x1="40.64" y1="101.6" x2="40.64" y2="124.46" width="0.1524" layer="91"/>
<wire x1="40.64" y1="124.46" x2="40.64" y2="149.86" width="0.1524" layer="91"/>
<wire x1="40.64" y1="149.86" x2="40.64" y2="152.4" width="0.1524" layer="91"/>
<wire x1="35.56" y1="78.74" x2="35.56" y2="76.2" width="0.1524" layer="91"/>
<wire x1="35.56" y1="78.74" x2="40.64" y2="78.74" width="0.1524" layer="91"/>
<junction x="40.64" y="78.74"/>
<pinref part="R105" gate="G$1" pin="2"/>
<wire x1="35.56" y1="101.6" x2="35.56" y2="99.06" width="0.1524" layer="91"/>
<wire x1="35.56" y1="101.6" x2="40.64" y2="101.6" width="0.1524" layer="91"/>
<junction x="40.64" y="101.6"/>
<pinref part="R103" gate="G$1" pin="2"/>
<wire x1="35.56" y1="124.46" x2="35.56" y2="121.92" width="0.1524" layer="91"/>
<wire x1="35.56" y1="124.46" x2="40.64" y2="124.46" width="0.1524" layer="91"/>
<junction x="40.64" y="124.46"/>
<pinref part="R101" gate="G$1" pin="2"/>
<wire x1="35.56" y1="149.86" x2="35.56" y2="147.32" width="0.1524" layer="91"/>
<wire x1="35.56" y1="149.86" x2="40.64" y2="149.86" width="0.1524" layer="91"/>
<junction x="40.64" y="149.86"/>
<pinref part="R111" gate="G$1" pin="2"/>
<wire x1="35.56" y1="27.94" x2="35.56" y2="30.48" width="0.1524" layer="91"/>
<wire x1="35.56" y1="30.48" x2="40.64" y2="30.48" width="0.1524" layer="91"/>
<wire x1="40.64" y1="30.48" x2="40.64" y2="55.88" width="0.1524" layer="91"/>
<junction x="40.64" y="55.88"/>
</segment>
<segment>
<pinref part="+3V3" gate="G$1" pin="+3V3"/>
<pinref part="U2" gate="P" pin="VCC"/>
<wire x1="5.08" y1="35.56" x2="5.08" y2="33.02" width="0.1524" layer="91"/>
<pinref part="C16" gate="G$1" pin="1"/>
<wire x1="5.08" y1="33.02" x2="5.08" y2="30.48" width="0.1524" layer="91"/>
<wire x1="12.7" y1="22.86" x2="12.7" y2="33.02" width="0.1524" layer="91"/>
<wire x1="12.7" y1="33.02" x2="5.08" y2="33.02" width="0.1524" layer="91"/>
<junction x="5.08" y="33.02"/>
</segment>
<segment>
<pinref part="+3V5" gate="G$1" pin="+3V3"/>
<pinref part="R4" gate="G$1" pin="2"/>
<wire x1="124.46" y1="96.52" x2="124.46" y2="93.98" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="U3" gate="G$1" pin="VOUT"/>
<pinref part="C9" gate="G$1" pin="1"/>
<wire x1="254" y1="177.8" x2="266.7" y2="177.8" width="0.1524" layer="91"/>
<wire x1="266.7" y1="177.8" x2="266.7" y2="170.18" width="0.1524" layer="91"/>
<pinref part="C10" gate="G$1" pin="1"/>
<wire x1="266.7" y1="177.8" x2="276.86" y2="177.8" width="0.1524" layer="91"/>
<wire x1="276.86" y1="177.8" x2="276.86" y2="170.18" width="0.1524" layer="91"/>
<junction x="266.7" y="177.8"/>
<pinref part="+3V6" gate="G$1" pin="+3V3"/>
<wire x1="276.86" y1="182.88" x2="276.86" y2="177.8" width="0.1524" layer="91"/>
<junction x="276.86" y="177.8"/>
<pinref part="R8" gate="G$1" pin="2"/>
<wire x1="276.86" y1="177.8" x2="289.56" y2="177.8" width="0.1524" layer="91"/>
<wire x1="289.56" y1="177.8" x2="289.56" y2="175.26" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="HALL" gate="A" pin="1"/>
<wire x1="134.62" y1="17.78" x2="142.24" y2="17.78" width="0.1524" layer="91"/>
<wire x1="142.24" y1="17.78" x2="142.24" y2="27.94" width="0.1524" layer="91"/>
<wire x1="142.24" y1="27.94" x2="147.32" y2="27.94" width="0.1524" layer="91"/>
<pinref part="+3V7" gate="G$1" pin="+3V3"/>
<pinref part="C15" gate="G$1" pin="1"/>
<wire x1="147.32" y1="27.94" x2="147.32" y2="30.48" width="0.1524" layer="91"/>
<wire x1="152.4" y1="22.86" x2="152.4" y2="27.94" width="0.1524" layer="91"/>
<wire x1="152.4" y1="27.94" x2="147.32" y2="27.94" width="0.1524" layer="91"/>
<junction x="147.32" y="27.94"/>
<pinref part="R5" gate="G$1" pin="2"/>
<wire x1="165.1" y1="25.4" x2="165.1" y2="27.94" width="0.1524" layer="91"/>
<wire x1="165.1" y1="27.94" x2="152.4" y2="27.94" width="0.1524" layer="91"/>
<junction x="152.4" y="27.94"/>
</segment>
<segment>
<pinref part="+3V8" gate="G$1" pin="+3V3"/>
<pinref part="R7" gate="G$1" pin="2"/>
<wire x1="195.58" y1="45.72" x2="195.58" y2="43.18" width="0.1524" layer="91"/>
</segment>
</net>
<net name="RS" class="0">
<segment>
<pinref part="LCD" gate="A" pin="10"/>
<wire x1="284.48" y1="33.02" x2="294.64" y2="33.02" width="0.1524" layer="91"/>
<label x="289.56" y="33.02" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="PB1"/>
<wire x1="170.18" y1="78.74" x2="162.56" y2="78.74" width="0.1524" layer="91"/>
<label x="162.56" y="78.74" size="1.778" layer="95"/>
</segment>
</net>
<net name="R/W" class="0">
<segment>
<pinref part="LCD" gate="A" pin="8"/>
<wire x1="284.48" y1="35.56" x2="294.64" y2="35.56" width="0.1524" layer="91"/>
<label x="289.56" y="35.56" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="PB10"/>
<wire x1="170.18" y1="73.66" x2="162.56" y2="73.66" width="0.1524" layer="91"/>
<label x="162.56" y="73.66" size="1.778" layer="95"/>
</segment>
</net>
<net name="E" class="0">
<segment>
<pinref part="LCD" gate="A" pin="6"/>
<wire x1="284.48" y1="38.1" x2="294.64" y2="38.1" width="0.1524" layer="91"/>
<label x="289.56" y="38.1" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="PB11"/>
<wire x1="162.56" y1="71.12" x2="170.18" y2="71.12" width="0.1524" layer="91"/>
<label x="162.56" y="71.12" size="1.778" layer="95"/>
</segment>
</net>
<net name="DB4" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB12"/>
<label x="223.52" y="66.04" size="1.778" layer="95"/>
<wire x1="236.22" y1="66.04" x2="220.98" y2="66.04" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="LCD" gate="A" pin="3"/>
<wire x1="276.86" y1="40.64" x2="266.7" y2="40.64" width="0.1524" layer="91"/>
<label x="266.7" y="40.64" size="1.778" layer="95"/>
</segment>
</net>
<net name="DB5" class="0">
<segment>
<pinref part="LCD" gate="A" pin="4"/>
<label x="289.56" y="40.64" size="1.778" layer="95"/>
<wire x1="284.48" y1="40.64" x2="294.64" y2="40.64" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="PB13"/>
<wire x1="241.3" y1="68.58" x2="220.98" y2="68.58" width="0.1524" layer="91"/>
<label x="223.52" y="68.58" size="1.778" layer="95"/>
</segment>
</net>
<net name="DB6" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB14"/>
<wire x1="220.98" y1="71.12" x2="233.68" y2="71.12" width="0.1524" layer="91"/>
<label x="223.52" y="71.12" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="LCD" gate="A" pin="1"/>
<wire x1="276.86" y1="43.18" x2="266.7" y2="43.18" width="0.1524" layer="91"/>
<label x="266.7" y="43.18" size="1.778" layer="95"/>
</segment>
</net>
<net name="DB7" class="0">
<segment>
<pinref part="LCD" gate="A" pin="2"/>
<label x="289.56" y="43.18" size="1.778" layer="95"/>
<wire x1="284.48" y1="43.18" x2="294.64" y2="43.18" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="U1" gate="G$1" pin="PB15"/>
<wire x1="238.76" y1="73.66" x2="220.98" y2="73.66" width="0.1524" layer="91"/>
<label x="223.52" y="73.66" size="1.778" layer="95"/>
</segment>
</net>
<net name="STEP" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA8"/>
<wire x1="220.98" y1="76.2" x2="231.14" y2="76.2" width="0.1524" layer="91"/>
<label x="223.52" y="76.2" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="DRV/PWR" gate="A" pin="6"/>
<wire x1="284.48" y1="66.04" x2="307.34" y2="66.04" width="0.1524" layer="91"/>
<label x="297.18" y="66.04" size="1.778" layer="95"/>
</segment>
</net>
<net name="DIR" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA9"/>
<label x="223.52" y="78.74" size="1.778" layer="95"/>
<wire x1="241.3" y1="78.74" x2="220.98" y2="78.74" width="0.1524" layer="91"/>
</segment>
<segment>
<pinref part="DRV/PWR" gate="A" pin="5"/>
<wire x1="276.86" y1="66.04" x2="264.16" y2="66.04" width="0.1524" layer="91"/>
<label x="264.16" y="66.04" size="1.778" layer="95"/>
</segment>
</net>
<net name="ENABLE" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA10"/>
<wire x1="220.98" y1="81.28" x2="236.22" y2="81.28" width="0.1524" layer="91"/>
<label x="223.52" y="81.28" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="DRV/PWR" gate="A" pin="4"/>
<wire x1="307.34" y1="68.58" x2="284.48" y2="68.58" width="0.1524" layer="91"/>
<label x="297.18" y="68.58" size="1.778" layer="95"/>
</segment>
</net>
<net name="RESET" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA11"/>
<wire x1="220.98" y1="83.82" x2="241.3" y2="83.82" width="0.1524" layer="91"/>
<label x="223.52" y="83.82" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="DRV/PWR" gate="A" pin="3"/>
<wire x1="264.16" y1="68.58" x2="276.86" y2="68.58" width="0.1524" layer="91"/>
<label x="264.16" y="68.58" size="1.778" layer="95"/>
</segment>
</net>
<net name="Q_SW" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA5"/>
<wire x1="170.18" y1="88.9" x2="162.56" y2="88.9" width="0.1524" layer="91"/>
<label x="162.56" y="88.9" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="D" pin="O"/>
<wire x1="88.9" y1="63.5" x2="99.06" y2="63.5" width="0.1524" layer="91"/>
<label x="91.44" y="63.5" size="1.778" layer="95"/>
</segment>
</net>
<net name="Q_DT" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA6"/>
<wire x1="170.18" y1="86.36" x2="162.56" y2="86.36" width="0.1524" layer="91"/>
<label x="162.56" y="86.36" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="E" pin="O"/>
<wire x1="88.9" y1="40.64" x2="101.6" y2="40.64" width="0.1524" layer="91"/>
<label x="91.44" y="40.64" size="1.778" layer="95"/>
</segment>
</net>
<net name="Q_CLK" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA7"/>
<wire x1="170.18" y1="83.82" x2="162.56" y2="83.82" width="0.1524" layer="91"/>
<label x="162.56" y="83.82" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="F" pin="O"/>
<wire x1="88.9" y1="15.24" x2="101.6" y2="15.24" width="0.1524" layer="91"/>
<label x="91.44" y="15.24" size="1.778" layer="95"/>
</segment>
</net>
<net name="SW_L" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA1"/>
<wire x1="170.18" y1="99.06" x2="162.56" y2="99.06" width="0.1524" layer="91"/>
<label x="162.56" y="99.06" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="A" pin="O"/>
<wire x1="88.9" y1="134.62" x2="99.06" y2="134.62" width="0.1524" layer="91"/>
<label x="91.44" y="134.62" size="1.778" layer="95"/>
</segment>
</net>
<net name="SW_R" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA2"/>
<wire x1="170.18" y1="96.52" x2="162.56" y2="96.52" width="0.1524" layer="91"/>
<label x="162.56" y="96.52" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="B" pin="O"/>
<wire x1="88.9" y1="109.22" x2="99.06" y2="109.22" width="0.1524" layer="91"/>
<label x="91.44" y="109.22" size="1.778" layer="95"/>
</segment>
</net>
<net name="NRST" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="NRST"/>
<pinref part="C14" gate="G$1" pin="1"/>
<wire x1="170.18" y1="109.22" x2="144.78" y2="109.22" width="0.1524" layer="91"/>
<wire x1="144.78" y1="109.22" x2="144.78" y2="106.68" width="0.1524" layer="91"/>
<label x="162.56" y="109.22" size="1.778" layer="95"/>
<pinref part="SW1" gate="G$1" pin="2"/>
<wire x1="144.78" y1="109.22" x2="137.16" y2="109.22" width="0.1524" layer="91"/>
<wire x1="137.16" y1="109.22" x2="137.16" y2="106.68" width="0.1524" layer="91"/>
<junction x="144.78" y="109.22"/>
</segment>
<segment>
<pinref part="STLINK" gate="A" pin="5"/>
<wire x1="276.86" y1="104.14" x2="264.16" y2="104.14" width="0.1524" layer="91"/>
<label x="264.16" y="104.14" size="1.778" layer="95"/>
</segment>
</net>
<net name="SWDIO" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA13/JTMS/SWDIO"/>
<wire x1="220.98" y1="88.9" x2="254" y2="88.9" width="0.1524" layer="91"/>
<wire x1="254" y1="88.9" x2="254" y2="106.68" width="0.1524" layer="91"/>
<pinref part="STLINK" gate="A" pin="4"/>
<wire x1="254" y1="106.68" x2="276.86" y2="106.68" width="0.1524" layer="91"/>
<label x="264.16" y="106.68" size="1.778" layer="95"/>
<label x="223.52" y="88.9" size="1.778" layer="95"/>
</segment>
</net>
<net name="SWCLK" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA14/JTCK/SWCLK"/>
<wire x1="220.98" y1="96.52" x2="251.46" y2="96.52" width="0.1524" layer="91"/>
<wire x1="251.46" y1="96.52" x2="251.46" y2="111.76" width="0.1524" layer="91"/>
<pinref part="STLINK" gate="A" pin="2"/>
<wire x1="251.46" y1="111.76" x2="276.86" y2="111.76" width="0.1524" layer="91"/>
<label x="223.52" y="96.52" size="1.778" layer="95"/>
<label x="264.16" y="111.76" size="1.778" layer="95"/>
</segment>
</net>
<net name="OSC_OUT" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="OSC_OUT"/>
<wire x1="170.18" y1="111.76" x2="157.48" y2="111.76" width="0.1524" layer="91"/>
<label x="157.48" y="111.76" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="R3" gate="G$1" pin="2"/>
<wire x1="142.24" y1="165.1" x2="142.24" y2="177.8" width="0.1524" layer="91"/>
<label x="142.24" y="165.1" size="1.778" layer="95" rot="R90"/>
</segment>
</net>
<net name="OSC_IN" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="OSC_IN"/>
<wire x1="170.18" y1="114.3" x2="157.48" y2="114.3" width="0.1524" layer="91"/>
<label x="157.48" y="114.3" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="Q1" gate="G$1" pin="2"/>
<pinref part="C12" gate="G$1" pin="2"/>
<wire x1="132.08" y1="152.4" x2="127" y2="152.4" width="0.1524" layer="91"/>
<wire x1="127" y1="152.4" x2="127" y2="149.86" width="0.1524" layer="91"/>
<wire x1="127" y1="152.4" x2="127" y2="167.64" width="0.1524" layer="91"/>
<junction x="127" y="152.4"/>
<label x="127" y="154.94" size="1.778" layer="95" rot="R90"/>
</segment>
</net>
<net name="BOOT0" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="BOOT0"/>
<pinref part="R2" gate="G$1" pin="1"/>
<wire x1="220.98" y1="114.3" x2="226.06" y2="114.3" width="0.1524" layer="91"/>
</segment>
</net>
<net name="HALL" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA0-WKUP"/>
<wire x1="170.18" y1="101.6" x2="162.56" y2="101.6" width="0.1524" layer="91"/>
<label x="162.56" y="101.6" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="HALL" gate="A" pin="3"/>
<wire x1="134.62" y1="12.7" x2="165.1" y2="12.7" width="0.1524" layer="91"/>
<label x="172.72" y="12.7" size="1.778" layer="95"/>
<pinref part="R5" gate="G$1" pin="1"/>
<wire x1="165.1" y1="12.7" x2="177.8" y2="12.7" width="0.1524" layer="91"/>
<wire x1="165.1" y1="15.24" x2="165.1" y2="12.7" width="0.1524" layer="91"/>
<junction x="165.1" y="12.7"/>
</segment>
</net>
<net name="SW_L_IN" class="0">
<segment>
<pinref part="R102" gate="G$1" pin="1"/>
<wire x1="43.18" y1="134.62" x2="35.56" y2="134.62" width="0.1524" layer="91"/>
<pinref part="R101" gate="G$1" pin="1"/>
<wire x1="35.56" y1="134.62" x2="25.4" y2="134.62" width="0.1524" layer="91"/>
<wire x1="35.56" y1="137.16" x2="35.56" y2="134.62" width="0.1524" layer="91"/>
<junction x="35.56" y="134.62"/>
<pinref part="CTRLS" gate="A" pin="1"/>
<wire x1="7.62" y1="91.44" x2="25.4" y2="91.44" width="0.1524" layer="91"/>
<wire x1="25.4" y1="91.44" x2="25.4" y2="134.62" width="0.1524" layer="91"/>
<label x="12.7" y="91.44" size="1.778" layer="95"/>
</segment>
</net>
<net name="N$3" class="0">
<segment>
<pinref part="R102" gate="G$1" pin="2"/>
<pinref part="C101" gate="G$1" pin="1"/>
<wire x1="53.34" y1="134.62" x2="55.88" y2="134.62" width="0.1524" layer="91"/>
<wire x1="55.88" y1="134.62" x2="55.88" y2="132.08" width="0.1524" layer="91"/>
<junction x="55.88" y="134.62"/>
<pinref part="U2" gate="A" pin="I"/>
<wire x1="55.88" y1="134.62" x2="68.58" y2="134.62" width="0.1524" layer="91"/>
</segment>
</net>
<net name="SW_R_IN" class="0">
<segment>
<pinref part="R104" gate="G$1" pin="1"/>
<wire x1="43.18" y1="109.22" x2="35.56" y2="109.22" width="0.1524" layer="91"/>
<pinref part="R103" gate="G$1" pin="1"/>
<wire x1="35.56" y1="109.22" x2="27.94" y2="109.22" width="0.1524" layer="91"/>
<wire x1="35.56" y1="111.76" x2="35.56" y2="109.22" width="0.1524" layer="91"/>
<junction x="35.56" y="109.22"/>
<pinref part="CTRLS" gate="A" pin="2"/>
<wire x1="7.62" y1="88.9" x2="27.94" y2="88.9" width="0.1524" layer="91"/>
<wire x1="27.94" y1="88.9" x2="27.94" y2="109.22" width="0.1524" layer="91"/>
<label x="12.7" y="88.9" size="1.778" layer="95"/>
</segment>
</net>
<net name="N$5" class="0">
<segment>
<pinref part="R104" gate="G$1" pin="2"/>
<pinref part="C102" gate="G$1" pin="1"/>
<wire x1="53.34" y1="109.22" x2="55.88" y2="109.22" width="0.1524" layer="91"/>
<wire x1="55.88" y1="109.22" x2="55.88" y2="106.68" width="0.1524" layer="91"/>
<wire x1="55.88" y1="109.22" x2="68.58" y2="109.22" width="0.1524" layer="91"/>
<junction x="55.88" y="109.22"/>
<pinref part="U2" gate="B" pin="I"/>
</segment>
</net>
<net name="SW_F_IN" class="0">
<segment>
<pinref part="R106" gate="G$1" pin="1"/>
<wire x1="43.18" y1="86.36" x2="35.56" y2="86.36" width="0.1524" layer="91"/>
<pinref part="R105" gate="G$1" pin="1"/>
<wire x1="35.56" y1="88.9" x2="35.56" y2="86.36" width="0.1524" layer="91"/>
<junction x="35.56" y="86.36"/>
<pinref part="CTRLS" gate="A" pin="3"/>
<wire x1="7.62" y1="86.36" x2="35.56" y2="86.36" width="0.1524" layer="91"/>
<label x="12.7" y="86.36" size="1.778" layer="95"/>
</segment>
</net>
<net name="N$7" class="0">
<segment>
<pinref part="R106" gate="G$1" pin="2"/>
<pinref part="C103" gate="G$1" pin="1"/>
<wire x1="53.34" y1="86.36" x2="55.88" y2="86.36" width="0.1524" layer="91"/>
<wire x1="55.88" y1="86.36" x2="55.88" y2="83.82" width="0.1524" layer="91"/>
<wire x1="55.88" y1="86.36" x2="68.58" y2="86.36" width="0.1524" layer="91"/>
<junction x="55.88" y="86.36"/>
<pinref part="U2" gate="C" pin="I"/>
</segment>
</net>
<net name="Q_SW_IN" class="0">
<segment>
<pinref part="R108" gate="G$1" pin="1"/>
<wire x1="43.18" y1="63.5" x2="35.56" y2="63.5" width="0.1524" layer="91"/>
<pinref part="R107" gate="G$1" pin="1"/>
<wire x1="35.56" y1="63.5" x2="27.94" y2="63.5" width="0.1524" layer="91"/>
<wire x1="35.56" y1="66.04" x2="35.56" y2="63.5" width="0.1524" layer="91"/>
<junction x="35.56" y="63.5"/>
<pinref part="ENCODER" gate="A" pin="1"/>
<wire x1="7.62" y1="58.42" x2="27.94" y2="58.42" width="0.1524" layer="91"/>
<wire x1="27.94" y1="58.42" x2="27.94" y2="63.5" width="0.1524" layer="91"/>
<label x="12.7" y="58.42" size="1.778" layer="95"/>
</segment>
</net>
<net name="N$9" class="0">
<segment>
<pinref part="R108" gate="G$1" pin="2"/>
<pinref part="C104" gate="G$1" pin="1"/>
<wire x1="53.34" y1="63.5" x2="55.88" y2="63.5" width="0.1524" layer="91"/>
<wire x1="55.88" y1="63.5" x2="55.88" y2="60.96" width="0.1524" layer="91"/>
<wire x1="55.88" y1="63.5" x2="68.58" y2="63.5" width="0.1524" layer="91"/>
<junction x="55.88" y="63.5"/>
<pinref part="U2" gate="D" pin="I"/>
</segment>
</net>
<net name="Q_DT_IN" class="0">
<segment>
<pinref part="R110" gate="G$1" pin="1"/>
<wire x1="43.18" y1="40.64" x2="35.56" y2="40.64" width="0.1524" layer="91"/>
<pinref part="R109" gate="G$1" pin="1"/>
<wire x1="35.56" y1="40.64" x2="27.94" y2="40.64" width="0.1524" layer="91"/>
<wire x1="35.56" y1="43.18" x2="35.56" y2="40.64" width="0.1524" layer="91"/>
<junction x="35.56" y="40.64"/>
<pinref part="ENCODER" gate="A" pin="2"/>
<wire x1="7.62" y1="55.88" x2="27.94" y2="55.88" width="0.1524" layer="91"/>
<wire x1="27.94" y1="55.88" x2="27.94" y2="40.64" width="0.1524" layer="91"/>
<label x="12.7" y="55.88" size="1.778" layer="95"/>
</segment>
</net>
<net name="N$11" class="0">
<segment>
<pinref part="R110" gate="G$1" pin="2"/>
<pinref part="C105" gate="G$1" pin="1"/>
<wire x1="53.34" y1="40.64" x2="55.88" y2="40.64" width="0.1524" layer="91"/>
<wire x1="55.88" y1="40.64" x2="55.88" y2="38.1" width="0.1524" layer="91"/>
<wire x1="55.88" y1="40.64" x2="68.58" y2="40.64" width="0.1524" layer="91"/>
<junction x="55.88" y="40.64"/>
<pinref part="U2" gate="E" pin="I"/>
</segment>
</net>
<net name="N$1" class="0">
<segment>
<pinref part="Q1" gate="G$1" pin="1"/>
<pinref part="C13" gate="G$1" pin="2"/>
<wire x1="137.16" y1="152.4" x2="142.24" y2="152.4" width="0.1524" layer="91"/>
<wire x1="142.24" y1="152.4" x2="142.24" y2="149.86" width="0.1524" layer="91"/>
<wire x1="142.24" y1="152.4" x2="142.24" y2="154.94" width="0.1524" layer="91"/>
<junction x="142.24" y="152.4"/>
<pinref part="R3" gate="G$1" pin="1"/>
</segment>
</net>
<net name="+5V" class="0">
<segment>
<pinref part="P+1" gate="1" pin="+5V"/>
<pinref part="C7" gate="G$1" pin="1"/>
<wire x1="200.66" y1="180.34" x2="200.66" y2="177.8" width="0.1524" layer="91"/>
<pinref part="C8" gate="G$1" pin="1"/>
<wire x1="200.66" y1="177.8" x2="200.66" y2="175.26" width="0.1524" layer="91"/>
<wire x1="210.82" y1="175.26" x2="210.82" y2="177.8" width="0.1524" layer="91"/>
<wire x1="210.82" y1="177.8" x2="200.66" y2="177.8" width="0.1524" layer="91"/>
<junction x="200.66" y="177.8"/>
<pinref part="U3" gate="G$1" pin="VIN"/>
<wire x1="226.06" y1="177.8" x2="223.52" y2="177.8" width="0.1524" layer="91"/>
<junction x="210.82" y="177.8"/>
<pinref part="U3" gate="G$1" pin="EN"/>
<wire x1="223.52" y1="177.8" x2="210.82" y2="177.8" width="0.1524" layer="91"/>
<wire x1="226.06" y1="172.72" x2="223.52" y2="172.72" width="0.1524" layer="91"/>
<wire x1="223.52" y1="172.72" x2="223.52" y2="177.8" width="0.1524" layer="91"/>
<junction x="223.52" y="177.8"/>
</segment>
<segment>
<pinref part="P+2" gate="1" pin="+5V"/>
<wire x1="264.16" y1="81.28" x2="264.16" y2="71.12" width="0.1524" layer="91"/>
<pinref part="DRV/PWR" gate="A" pin="1"/>
<wire x1="264.16" y1="71.12" x2="276.86" y2="71.12" width="0.1524" layer="91"/>
</segment>
<segment>
<wire x1="261.62" y1="38.1" x2="261.62" y2="43.18" width="0.1524" layer="91"/>
<pinref part="P+3" gate="1" pin="+5V"/>
<pinref part="LCD" gate="A" pin="5"/>
<wire x1="261.62" y1="43.18" x2="261.62" y2="48.26" width="0.1524" layer="91"/>
<wire x1="276.86" y1="38.1" x2="261.62" y2="38.1" width="0.1524" layer="91"/>
<pinref part="R6" gate="G$1" pin="E"/>
<wire x1="246.38" y1="40.64" x2="246.38" y2="43.18" width="0.1524" layer="91"/>
<wire x1="246.38" y1="43.18" x2="261.62" y2="43.18" width="0.1524" layer="91"/>
<junction x="261.62" y="43.18"/>
</segment>
</net>
<net name="N$12" class="0">
<segment>
<pinref part="U2" gate="F" pin="I"/>
<wire x1="68.58" y1="15.24" x2="55.88" y2="15.24" width="0.1524" layer="91"/>
<pinref part="R112" gate="G$1" pin="2"/>
<pinref part="C106" gate="G$1" pin="1"/>
<wire x1="55.88" y1="15.24" x2="53.34" y2="15.24" width="0.1524" layer="91"/>
<wire x1="55.88" y1="12.7" x2="55.88" y2="15.24" width="0.1524" layer="91"/>
<junction x="55.88" y="15.24"/>
</segment>
</net>
<net name="Q_CLK_IN" class="0">
<segment>
<pinref part="ENCODER" gate="A" pin="3"/>
<wire x1="7.62" y1="53.34" x2="25.4" y2="53.34" width="0.1524" layer="91"/>
<wire x1="25.4" y1="53.34" x2="25.4" y2="15.24" width="0.1524" layer="91"/>
<pinref part="R112" gate="G$1" pin="1"/>
<wire x1="25.4" y1="15.24" x2="35.56" y2="15.24" width="0.1524" layer="91"/>
<pinref part="R111" gate="G$1" pin="1"/>
<wire x1="35.56" y1="15.24" x2="43.18" y2="15.24" width="0.1524" layer="91"/>
<wire x1="35.56" y1="17.78" x2="35.56" y2="15.24" width="0.1524" layer="91"/>
<junction x="35.56" y="15.24"/>
<label x="12.7" y="53.34" size="1.778" layer="95"/>
</segment>
</net>
<net name="SW_F" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA3"/>
<wire x1="170.18" y1="93.98" x2="162.56" y2="93.98" width="0.1524" layer="91"/>
<label x="162.56" y="93.98" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="U2" gate="C" pin="O"/>
<wire x1="88.9" y1="86.36" x2="99.06" y2="86.36" width="0.1524" layer="91"/>
<label x="91.44" y="86.36" size="1.778" layer="95"/>
</segment>
</net>
<net name="BOOT1" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB2/BOOT1"/>
<pinref part="R1" gate="G$1" pin="2"/>
<wire x1="152.4" y1="76.2" x2="170.18" y2="76.2" width="0.1524" layer="91"/>
<label x="162.56" y="76.2" size="1.778" layer="95"/>
</segment>
</net>
<net name="ESTOP" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PB0"/>
<label x="162.56" y="81.28" size="1.778" layer="95"/>
<wire x1="170.18" y1="81.28" x2="124.46" y2="81.28" width="0.1524" layer="91"/>
<pinref part="ESTOP" gate="G$1" pin="1"/>
<pinref part="R4" gate="G$1" pin="1"/>
<wire x1="124.46" y1="81.28" x2="109.22" y2="81.28" width="0.1524" layer="91"/>
<wire x1="124.46" y1="83.82" x2="124.46" y2="81.28" width="0.1524" layer="91"/>
<junction x="124.46" y="81.28"/>
</segment>
</net>
<net name="BP" class="0">
<segment>
<pinref part="U3" gate="G$1" pin="BP"/>
<pinref part="C11" gate="G$1" pin="1"/>
<wire x1="254" y1="165.1" x2="256.54" y2="165.1" width="0.1524" layer="91"/>
<wire x1="256.54" y1="165.1" x2="256.54" y2="162.56" width="0.1524" layer="91"/>
</segment>
</net>
<net name="V0" class="0">
<segment>
<pinref part="LCD" gate="A" pin="7"/>
<pinref part="R6" gate="G$1" pin="S"/>
<wire x1="276.86" y1="35.56" x2="251.46" y2="35.56" width="0.1524" layer="91"/>
<label x="266.7" y="35.56" size="1.778" layer="95"/>
</segment>
</net>
<net name="LED" class="0">
<segment>
<pinref part="U1" gate="G$1" pin="PA4"/>
<wire x1="170.18" y1="91.44" x2="162.56" y2="91.44" width="0.1524" layer="91"/>
<label x="162.56" y="91.44" size="1.778" layer="95"/>
</segment>
<segment>
<pinref part="D1" gate="G$1" pin="C"/>
<wire x1="195.58" y1="22.86" x2="195.58" y2="12.7" width="0.1524" layer="91"/>
<label x="195.58" y="12.7" size="1.778" layer="95" rot="R90"/>
</segment>
</net>
<net name="N$2" class="0">
<segment>
<pinref part="R7" gate="G$1" pin="1"/>
<pinref part="D1" gate="G$1" pin="A"/>
<wire x1="195.58" y1="33.02" x2="195.58" y2="30.48" width="0.1524" layer="91"/>
</segment>
</net>
<net name="N$4" class="0">
<segment>
<pinref part="R8" gate="G$1" pin="1"/>
<pinref part="D2" gate="G$1" pin="A"/>
<wire x1="289.56" y1="165.1" x2="289.56" y2="162.56" width="0.1524" layer="91"/>
</segment>
</net>
</nets>
</sheet>
</sheets>
</schematic>
</drawing>
</eagle>
