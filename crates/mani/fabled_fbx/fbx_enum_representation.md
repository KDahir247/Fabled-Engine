# FBX Enum Representation

--- 

## Time Mode

---

* 0 = FBX Enum Representation
* 1 = Time Mode
* 2 = eDEFAULT_MODE
* 3 = eFRAMES120120 frames/s
* 4 = eFRAMES100100 frames/s
* 5 = eFRAMES6060 frames/s
* 6 = eFRAMES5050 frames/s
* 7 = eFRAMES4848 frame/s
* 8 = eFRAMES3030 frames/s BLACK&WHITE NTSC
* 9 = eFRAMES30_DROP30 frames/s use when diplay in frame is selected(equivalent to NTSC_DROP)
* 10 = eNTSC_DROP_FRAME29.97002617 frames/s drop COLOR NTSC
* 11 = eNTSC_FULL_FRAME29.97002617 frames/s COLOR NTSC
* 12 = ePAL25 frames/s PAL/SECAM
* 13 = eCINEMA24 frames/s
* 14 = eFRAMES10001000 milli/s (use for datetime)
* 15 = eCINEMA_ND23.976 frames/s
* 16 = eCUSTOMCustom Framerate value



## AspectRatio Mode

--- 

* 0 = eWindowSize
* 1 = eFixedRatio
* 2 = eFixedResolution
* 3 = eFixedWidth
* 4 = eFixedHeight



## Film Format Index

--- 
* 0 = eCustomAperture
* 1 = e16mmTheatrical
* 2 = eSuper16mm
* 3 = e35mmAcademy
* 4 = e35mmTVProjection
* 5 = e35mmFullAperture
* 6 = e35mm185Projection
* 7 = e35mmAnamorphic
* 8 = e70mmProjection
* 9 = eVistaVision
* 10 = eDynaVision
* 11 = eIMax


## FillRoll Order

---

* 0 = eRotateFirst
* 1 = eTranslateFirst

## Aperture Mode

--- 

* 0 = eHorizAndVert
* 1 = eHorizontal
* 2 = eVertical
* 3 = eFocalLength

## Gate Fit

--- 

* 0 = eFitNone
* 1 = eFitVertical
* 2 = eFitHorizontal
* 3 = eFitFill
* 4 = eFitOverscan
* 5 = eFitStretch

## CameraFormat

--- 

* 0 = eCustomFormat
* 1 = eD1NTSC
* 2 = eNTSC
* 3 = ePAL
* 4 = eD1PAL
* 5 = eHD
* 6 = e640x480
* 7 = e320x200
* 8 = e128x128
* 9 = eFullscreen


## ViewFrustumBackPlaneMode

--- 

* 0 = ePlanesDisabled
* 1 = ePlanesAlways
* 2 = ePlanesWhenMedia

## BackPlaneDistanceMode/FrontPlaneDistanceMode

---

* 0 = eRelativeToInterest
* 1 = eRelativeToCamera

## ViewFrustumFrontPlaneMode

--- 

* 0 = ePlanesDisabled
* 1 = ePlanesAlways
* 2 = ePlanesWhenMedia

## SafeAreaDisplayStyle

--- 

* 0 = eSafeAreaRound
* 1 = eSafeAreaSquare

## CameraProjectionType

---

* 0 = ePERSPECTIVE
* 1 = eORTHOGONAL

## AntialiasingMethod

---

* 0 = eAAOversampling
* 1 = eAAHardware

## FrameSamplingType

--- 

* 0 = eSamplingUniform
* 1 = eSamplingStochastic

##  QuaternionInterpolate

---

* 0 = eQuatInterpOff
* 1 = eQuatInterpClassic
* 2 = eQuatInterpSlerp
* 3 = eQuatInterpCubic
* 4 = eQuatInterpTangentDependent
* 5 = eQuatInterpCount

## RotationOrder

---

* 0 = eEulerXYZ
* 1 = eEulerXZY
* 2 = eEulerYZX
* 3 = eEulerYXZ
* 4 = eEulerZXY
* 5 = eEulerZYX
* 6 = eSphericXYZ

## InheritType

--- 

* 0 = eInheritRrSs
* 1 = eInheritRSrs
* 2 = eInheritRrs

## LightType

---

* 0 = ePoint
* 1 = eDirectional
* 2 = eSpot
* 3 = eArea
* 4 = eVolume

## DecayType

---

* 0 = eNone
* 1 = eLinear
* 2 = eQuadratic
* 3 = eCubic

## AreaLightShape

---

* 0 = eRectangle
* 1 = eSphere

## BackgroundMode

---

* 0 = eBACKGROUNDImage
* 1 = eFOREGROUNDImage
* 2 = eBACKGROUND_AND_FOREGROUNDImage

