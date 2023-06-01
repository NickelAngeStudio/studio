// Generated with "script/rustify_x11_constants.sh"
use std::os::raw::{ c_int, c_long };

/* Definitions for the X window system likely to be used by applications */
/***********************************************************
******************************************************************/
pub const X_PROTOCOL:c_int=11;
pub const X_PROTOCOL_REVISION:c_int=0;
/* Resources */
/*
* _XSERVER64 must ONLY be defined when compiling X server sources on
* systems where unsigned long is not 32 bits, must NOT be used in
* client or library code.
*/
/*****************************************************************
* RESERVED RESOURCE AND CONSTANT DEFINITIONS
*****************************************************************/
pub const None:c_long=0;
pub const ParentRelative:c_long=0;
pub const CopyFromParent:c_long=0;
pub const PointerWindow:c_long=0;
pub const InputFocus:c_long=0;
pub const PointerRoot:c_long=0;
pub const AnyPropertyType:c_long=0;
pub const AnyKey:c_long=0;
pub const AnyButton:c_long=0;
pub const AllTemporary:c_long=0;
pub const CurrentTime:c_long=0;
pub const NoSymbol:c_long=0;
/*****************************************************************
* EVENT DEFINITIONS
*****************************************************************/
/* Input Event Masks. Used as event-mask window attribute and as arguments
to Grab requests.  Not to be confused with event names.  */
pub const NoEventMask:c_long=0;
pub const KeyPressMask:c_long=1 << 0;
pub const KeyReleaseMask:c_long=1 << 1;
pub const ButtonPressMask:c_long=1 << 2;
pub const ButtonReleaseMask:c_long=1 << 3;
pub const EnterWindowMask:c_long=1 << 4;
pub const LeaveWindowMask:c_long=1 << 5;
pub const PointerMotionMask:c_long=1 << 6;
pub const PointerMotionHintMask:c_long=1 << 7;
pub const Button1MotionMask:c_long=1 << 8;
pub const Button2MotionMask:c_long=1 << 9;
pub const Button3MotionMask:c_long=1 << 10;
pub const Button4MotionMask:c_long=1 << 11;
pub const Button5MotionMask:c_long=1 << 12;
pub const ButtonMotionMask:c_long=1 << 13;
pub const KeymapStateMask:c_long=1 << 14;
pub const ExposureMask:c_long=1 << 15;
pub const VisibilityChangeMask:c_long=1 << 16;
pub const StructureNotifyMask:c_long=1 << 17;
pub const ResizeRedirectMask:c_long=1 << 18;
pub const SubstructureNotifyMask:c_long=1 << 19;
pub const SubstructureRedirectMask:c_long=1 << 20;
pub const FocusChangeMask:c_long=1 << 21;
pub const PropertyChangeMask:c_long=1 << 22;
pub const ColormapChangeMask:c_long=1 << 23;
pub const OwnerGrabButtonMask:c_long=1 << 24;
/* Event names.  Used in "type" field in XEvent structures.  Not to be
are reserved in the protocol for errors and replies. */
pub const KeyPress:c_int=2;
pub const KeyRelease:c_int=3;
pub const ButtonPress:c_int=4;
pub const ButtonRelease:c_int=5;
pub const MotionNotify:c_int=6;
pub const EnterNotify:c_int=7;
pub const LeaveNotify:c_int=8;
pub const FocusIn:c_int=9;
pub const FocusOut:c_int=10;
pub const KeymapNotify:c_int=11;
pub const Expose:c_int=12;
pub const GraphicsExpose:c_int=13;
pub const NoExpose:c_int=14;
pub const VisibilityNotify:c_int=15;
pub const CreateNotify:c_int=16;
pub const DestroyNotify:c_int=17;
pub const UnmapNotify:c_int=18;
pub const MapNotify:c_int=19;
pub const MapRequest:c_int=20;
pub const ReparentNotify:c_int=21;
pub const ConfigureNotify:c_int=22;
pub const ConfigureRequest:c_int=23;
pub const GravityNotify:c_int=24;
pub const ResizeRequest:c_int=25;
pub const CirculateNotify:c_int=26;
pub const CirculateRequest:c_int=27;
pub const PropertyNotify:c_int=28;
pub const SelectionClear:c_int=29;
pub const SelectionRequest:c_int=30;
pub const SelectionNotify:c_int=31;
pub const ColormapNotify:c_int=32;
pub const ClientMessage:c_int=33;
pub const MappingNotify:c_int=34;
pub const GenericEvent:c_int=35;
pub const LASTEvent:c_int=36;
/* Key masks. Used as modifiers to GrabButton and GrabKey, results of QueryPointer,
state in various key-, mouse-, and button-related events. */
pub const ShiftMask:c_long=1 << 0;
pub const LockMask:c_long=1 << 1;
pub const ControlMask:c_long=1 << 2;
pub const Mod1Mask:c_long=1 << 3;
pub const Mod2Mask:c_long=1 << 4;
pub const Mod3Mask:c_long=1 << 5;
pub const Mod4Mask:c_long=1 << 6;
pub const Mod5Mask:c_long=1 << 7;
/* modifier names.  Used to build a SetModifierMapping request or
masks defined above. */
pub const ShiftMapIndex:c_int=0;
pub const LockMapIndex:c_int=1;
pub const ControlMapIndex:c_int=2;
pub const Mod1MapIndex:c_int=3;
pub const Mod2MapIndex:c_int=4;
pub const Mod3MapIndex:c_int=5;
pub const Mod4MapIndex:c_int=6;
pub const Mod5MapIndex:c_int=7;
/* button masks.  Used in same manner as Key masks above. Not to be confused
with button names below. */
pub const Button1Mask:c_long=1 << 8;
pub const Button2Mask:c_long=1 << 9;
pub const Button3Mask:c_long=1 << 10;
pub const Button4Mask:c_long=1 << 11;
pub const Button5Mask:c_long=1 << 12;
pub const AnyModifier:c_long=1 << 15;
/* button names. Used as arguments to GrabButton and as detail in ButtonPress
Note that 0 is already defined above as "AnyButton".  */
pub const Button1:c_int=1;
pub const Button2:c_int=2;
pub const Button3:c_int=3;
pub const Button4:c_int=4;
pub const Button5:c_int=5;
/* Notify modes */
pub const NotifyNormal:c_int=0;
pub const NotifyGrab:c_int=1;
pub const NotifyUngrab:c_int=2;
pub const NotifyWhileGrabbed:c_int=3;
pub const NotifyHint:c_int=1;
/* Notify detail */
pub const NotifyAncestor:c_int=0;
pub const NotifyVirtual:c_int=1;
pub const NotifyInferior:c_int=2;
pub const NotifyNonlinear:c_int=3;
pub const NotifyNonlinearVirtual:c_int=4;
pub const NotifyPointer:c_int=5;
pub const NotifyPointerRoot:c_int=6;
pub const NotifyDetailNone:c_int=7;
/* Visibility notify */
pub const VisibilityUnobscured:c_int=0;
pub const VisibilityPartiallyObscured:c_int=1;
pub const VisibilityFullyObscured:c_int=2;
/* Circulation request */
pub const PlaceOnTop:c_int=0;
pub const PlaceOnBottom:c_int=1;
/* protocol families */
pub const FamilyInternet:c_int=0;
pub const FamilyDECnet:c_int=1;
pub const FamilyChaos:c_int=2;
pub const FamilyInternet6:c_int=6;
/* authentication families not tied to a specific protocol */
pub const FamilyServerInterpreted:c_int=5;
/* Property notification */
pub const PropertyNewValue:c_int=0;
pub const PropertyDelete:c_int=1;
/* Color Map notification */
pub const ColormapUninstalled:c_int=0;
pub const ColormapInstalled:c_int=1;
/* GrabPointer, GrabButton, GrabKeyboard, GrabKey Modes */
pub const GrabModeSync:c_int=0;
pub const GrabModeAsync:c_int=1;
/* GrabPointer, GrabKeyboard reply status */
pub const GrabSuccess:c_int=0;
pub const AlreadyGrabbed:c_int=1;
pub const GrabInvalidTime:c_int=2;
pub const GrabNotViewable:c_int=3;
pub const GrabFrozen:c_int=4;
/* AllowEvents modes */
pub const AsyncPointer:c_int=0;
pub const SyncPointer:c_int=1;
pub const ReplayPointer:c_int=2;
pub const AsyncKeyboard:c_int=3;
pub const SyncKeyboard:c_int=4;
pub const ReplayKeyboard:c_int=5;
pub const AsyncBoth:c_int=6;
pub const SyncBoth:c_int=7;
/* Used in SetInputFocus, GetInputFocus */
pub const RevertToNone:c_int=None as i32;
pub const RevertToPointerRoot:c_int=PointerRoot as i32;
pub const RevertToParent:c_int=2;
/*****************************************************************
* ERROR CODES
*****************************************************************/
pub const Success:c_int=0;
pub const BadRequest:c_int=1;
pub const BadValue:c_int=2;
pub const BadWindow:c_int=3;
pub const BadPixmap:c_int=4;
pub const BadAtom:c_int=5;
pub const BadCursor:c_int=6;
pub const BadFont:c_int=7;
pub const BadMatch:c_int=8;
pub const BadDrawable:c_int=9;
pub const BadAccess:c_int=10;
pub const BadAlloc:c_int=11;
pub const BadColor:c_int=12;
pub const BadGC:c_int=13;
pub const BadIDChoice:c_int=14;
pub const BadName:c_int=15;
pub const BadLength:c_int=16;
pub const BadImplementation:c_int=17;
pub const FirstExtensionError:c_int=128;
pub const LastExtensionError:c_int=255;
/*****************************************************************
* WINDOW DEFINITIONS
*****************************************************************/
/* Window classes used by CreateWindow */
/* Note that CopyFromParent is already defined as 0 above */
pub const InputOutput:c_int=1;
pub const InputOnly:c_int=2;
/* Window attributes for CreateWindow and ChangeWindowAttributes */
pub const CWBackPixmap:c_long=1 << 0;
pub const CWBackPixel:c_long=1 << 1;
pub const CWBorderPixmap:c_long=1 << 2;
pub const CWBorderPixel:c_long=1 << 3;
pub const CWBitGravity:c_long=1 << 4;
pub const CWWinGravity:c_long=1 << 5;
pub const CWBackingStore:c_long=1 << 6;
pub const CWBackingPlanes:c_long=1 << 7;
pub const CWBackingPixel:c_long=1 << 8;
pub const CWOverrideRedirect:c_long=1 << 9;
pub const CWSaveUnder:c_long=1 << 10;
pub const CWEventMask:c_long=1 << 11;
pub const CWDontPropagate:c_long=1 << 12;
pub const CWColormap:c_long=1 << 13;
pub const CWCursor:c_long=1 << 14;
/* ConfigureWindow structure */
pub const CWX:c_long=1 << 0;
pub const CWY:c_long=1 << 1;
pub const CWWidth:c_long=1 << 2;
pub const CWHeight:c_long=1 << 3;
pub const CWBorderWidth:c_long=1 << 4;
pub const CWSibling:c_long=1 << 5;
pub const CWStackMode:c_long=1 << 6;
/* Bit Gravity */
pub const ForgetGravity:c_int=0;
pub const NorthWestGravity:c_int=1;
pub const NorthGravity:c_int=2;
pub const NorthEastGravity:c_int=3;
pub const WestGravity:c_int=4;
pub const CenterGravity:c_int=5;
pub const EastGravity:c_int=6;
pub const SouthWestGravity:c_int=7;
pub const SouthGravity:c_int=8;
pub const SouthEastGravity:c_int=9;
pub const StaticGravity:c_int=10;
/* Window gravity + bit gravity above */
pub const UnmapGravity:c_int=0;
/* Used in CreateWindow for backing-store hint */
pub const NotUseful:c_int=0;
pub const WhenMapped:c_int=1;
pub const Always:c_int=2;
/* Used in GetWindowAttributes reply */
pub const IsUnmapped:c_int=0;
pub const IsUnviewable:c_int=1;
pub const IsViewable:c_int=2;
/* Used in ChangeSaveSet */
pub const SetModeInsert:c_int=0;
pub const SetModeDelete:c_int=1;
/* Used in ChangeCloseDownMode */
pub const DestroyAll:c_int=0;
pub const RetainPermanent:c_int=1;
pub const RetainTemporary:c_int=2;
/* Window stacking method (in configureWindow) */
pub const Above:c_int=0;
pub const Below:c_int=1;
pub const TopIf:c_int=2;
pub const BottomIf:c_int=3;
pub const Opposite:c_int=4;
/* Circulation direction */
pub const RaiseLowest:c_int=0;
pub const LowerHighest:c_int=1;
/* Property modes */
pub const PropModeReplace:c_int=0;
pub const PropModePrepend:c_int=1;
pub const PropModeAppend:c_int=2;
/*****************************************************************
* GRAPHICS DEFINITIONS
*****************************************************************/
/* graphics functions, as in GC.alu */
pub const GXclear:c_int=0x0;
pub const GXand:c_int=0x1;
pub const GXandReverse:c_int=0x2;
pub const GXcopy:c_int=0x3;
pub const GXandInverted:c_int=0x4;
pub const GXnoop:c_int=0x5;
pub const GXxor:c_int=0x6;
pub const GXor:c_int=0x7;
pub const GXnor:c_int=0x8;
pub const GXequiv:c_int=0x9;
pub const GXinvert:c_int=0xa;
pub const GXorReverse:c_int=0xb;
pub const GXcopyInverted:c_int=0xc;
pub const GXorInverted:c_int=0xd;
pub const GXnand:c_int=0xe;
pub const GXset:c_int=0xf;
/* LineStyle */
pub const LineSolid:c_int=0;
pub const LineOnOffDash:c_int=1;
pub const LineDoubleDash:c_int=2;
/* capStyle */
pub const CapNotLast:c_int=0;
pub const CapButt:c_int=1;
pub const CapRound:c_int=2;
pub const CapProjecting:c_int=3;
/* joinStyle */
pub const JoinMiter:c_int=0;
pub const JoinRound:c_int=1;
pub const JoinBevel:c_int=2;
/* fillStyle */
pub const FillSolid:c_int=0;
pub const FillTiled:c_int=1;
pub const FillStippled:c_int=2;
pub const FillOpaqueStippled:c_int=3;
/* fillRule */
pub const EvenOddRule:c_int=0;
pub const WindingRule:c_int=1;
/* subwindow mode */
pub const ClipByChildren:c_int=0;
pub const IncludeInferiors:c_int=1;
/* SetClipRectangles ordering */
pub const Unsorted:c_int=0;
pub const YSorted:c_int=1;
pub const YXSorted:c_int=2;
pub const YXBanded:c_int=3;
/* CoordinateMode for drawing routines */
pub const CoordModeOrigin:c_int=0;
pub const CoordModePrevious:c_int=1;
/* Polygon shapes */
pub const Complex:c_int=0;
pub const Nonconvex:c_int=1;
pub const Convex:c_int=2;
/* Arc modes for PolyFillArc */
pub const ArcChord:c_int=0;
pub const ArcPieSlice:c_int=1;
/* GC components: masks used in CreateGC, CopyGC, ChangeGC, OR'ed into
GC.stateChanges */
pub const GCFunction:c_long=1 << 0;
pub const GCPlaneMask:c_long=1 << 1;
pub const GCForeground:c_long=1 << 2;
pub const GCBackground:c_long=1 << 3;
pub const GCLineWidth:c_long=1 << 4;
pub const GCLineStyle:c_long=1 << 5;
pub const GCCapStyle:c_long=1 << 6;
pub const GCJoinStyle:c_long=1 << 7;
pub const GCFillStyle:c_long=1 << 8;
pub const GCFillRule:c_long=1 << 9;
pub const GCTile:c_long=1 << 10;
pub const GCStipple:c_long=1 << 11;
pub const GCTileStipXOrigin:c_long=1 << 12;
pub const GCTileStipYOrigin:c_long=1 << 13;
pub const GCFont:c_long=1 << 14;
pub const GCSubwindowMode:c_long=1 << 15;
pub const GCGraphicsExposures:c_long=1 << 16;
pub const GCClipXOrigin:c_long=1 << 17;
pub const GCClipYOrigin:c_long=1 << 18;
pub const GCClipMask:c_long=1 << 19;
pub const GCDashOffset:c_long=1 << 20;
pub const GCDashList:c_long=1 << 21;
pub const GCArcMode:c_long=1 << 22;
pub const GCLastBit:c_int=22;
/*****************************************************************
* FONTS
*****************************************************************/
/* used in QueryFont -- draw direction */
pub const FontLeftToRight:c_int=0;
pub const FontRightToLeft:c_int=1;
pub const FontChange:c_int=255;
/*****************************************************************
*  IMAGING
*****************************************************************/
/* ImageFormat -- PutImage, GetImage */
pub const XYBitmap:c_int=0;
pub const XYPixmap:c_int=1;
pub const ZPixmap:c_int=2;
/*****************************************************************
*  COLOR MAP STUFF
*****************************************************************/
/* For CreateColormap */
pub const AllocNone:c_int=0;
pub const AllocAll:c_int=1;
/* Flags used in StoreNamedColor, StoreColors */
pub const DoRed:c_long=1 << 0;
pub const DoGreen:c_long=1 << 1;
pub const DoBlue:c_long=1 << 2;
/*****************************************************************
* CURSOR STUFF
*****************************************************************/
/* QueryBestSize Class */
pub const CursorShape:c_int=0;
pub const TileShape:c_int=1;
pub const StippleShape:c_int=2;
/*****************************************************************
* KEYBOARD/POINTER STUFF
*****************************************************************/
pub const AutoRepeatModeOff:c_int=0;
pub const AutoRepeatModeOn:c_int=1;
pub const AutoRepeatModeDefault:c_int=2;
pub const LedModeOff:c_int=0;
pub const LedModeOn:c_int=1;
/* masks for ChangeKeyboardControl */
pub const KBKeyClickPercent:c_long=1 << 0;
pub const KBBellPercent:c_long=1 << 1;
pub const KBBellPitch:c_long=1 << 2;
pub const KBBellDuration:c_long=1 << 3;
pub const KBLed:c_long=1 << 4;
pub const KBLedMode:c_long=1 << 5;
pub const KBKey:c_long=1 << 6;
pub const KBAutoRepeatMode:c_long=1 << 7;
pub const MappingSuccess:c_int=0;
pub const MappingBusy:c_int=1;
pub const MappingFailed:c_int=2;
pub const MappingModifier:c_int=0;
pub const MappingKeyboard:c_int=1;
pub const MappingPointer:c_int=2;
/*****************************************************************
* SCREEN SAVER STUFF
*****************************************************************/
pub const DontPreferBlanking:c_int=0;
pub const PreferBlanking:c_int=1;
pub const DefaultBlanking:c_int=2;
pub const DisableScreenSaver:c_int=0;
pub const DisableScreenInterval:c_int=0;
pub const DontAllowExposures:c_int=0;
pub const AllowExposures:c_int=1;
pub const DefaultExposures:c_int=2;
/* for ForceScreenSaver */
pub const ScreenSaverReset:c_int=0;
pub const ScreenSaverActive:c_int=1;
/*****************************************************************
* HOSTS AND CONNECTIONS
*****************************************************************/
/* for ChangeHosts */
pub const HostInsert:c_int=0;
pub const HostDelete:c_int=1;
/* for ChangeAccessControl */
pub const EnableAccess:c_int=1;
pub const DisableAccess:c_int=0;
/* Display classes  used in opening the connection
* Note that the statically allocated ones are even numbered and the
* dynamically changeable ones are odd numbered */
pub const StaticGray:c_int=0;
pub const GrayScale:c_int=1;
pub const StaticColor:c_int=2;
pub const PseudoColor:c_int=3;
pub const TrueColor:c_int=4;
pub const DirectColor:c_int=5;
/* Byte order  used in imageByteOrder and bitmapBitOrder */
pub const LSBFirst:c_int=0;
pub const MSBFirst:c_int=1;