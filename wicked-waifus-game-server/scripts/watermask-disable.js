const UE = require("ue");
const Info_1 = require("../../../Core/Common/Info");
const MathUtils_1 = require("../../../Core/Utils/MathUtils");
const UiLayerType_1 = require("../../Ui/Define/UiLayerType");
const UiLayer_1 = require("../../Ui/UiLayer");

var _a = require("../Module/WaterMask/WaterMaskController").WaterMaskView;

_a.LOo = 0.05;
_a.yOo = 700;
_a.IOo = 700;

let colorCycle = [
    "#0000FF", // 深蓝
    "#0033FF",
    "#0066FF",
    "#0099FF",
    "#00CCFF",
    "#33DFFF",
    "#66EFFF", // 浅蓝
    "#99FFFF"  // 更浅的蓝
];
let colorIndex = 0;
let directionX = 3;
let directionY = 3;
const SPEED = 8;
const TEXT_OFFSET_X = 270;
const TEXT_OFFSET_Y = 20;

_a.vOo = function () {
    void 0 !== _a.SOo && _a.EOo();
    let e = UiLayer_1.UiLayer.GetLayerRootUiItem(
        UiLayerType_1.ELayerType.WaterMask,
    );
    _a.SOo = UE.KuroActorManager.SpawnActor(
        Info_1.Info.World,
        UE.UIContainerActor.StaticClass(),
        MathUtils_1.MathUtils.DefaultTransform,
        void 0,
    );
    let t = _a.SOo.RootComponent;
    t.SetDisplayName("WaterMaskContainer");
    UE.KuroStaticLibrary.SetActorPermanent(_a.SOo, !0, !0);
    _a.SOo.K2_AttachRootComponentTo(e);
    
    let rootComponent = t.GetRootCanvas().GetOwner().RootComponent;
    let screenWidth = rootComponent.widget.width;
    let screenHeight = rootComponent.widget.height;
    
    let textActor = UE.KuroActorManager.SpawnActor(
        Info_1.Info.World,
        UE.UITextActor.StaticClass(),
        MathUtils_1.MathUtils.DefaultTransform,
        void 0,
    );
    let textRoot = textActor.RootComponent;
    textActor.K2_AttachRootComponentTo(t);
    textRoot.SetDisplayName("WuWa Start");
    let textComponent = textActor.GetComponentByClass(UE.UIText.StaticClass());
    
    textComponent.SetFontSize(48);
    textComponent.SetOverflowType(0);
    textComponent.SetAlpha(_a.LOo);
    textComponent.SetFont(UE.LGUIFontData.GetDefaultFont());
    textComponent.SetText("玩原神玩的");
    textComponent.SetColor(UE.Color.FromHex(colorCycle[colorIndex]));
    
    let posX = 0;
    let posY = 0;
    const halfWidth = screenWidth / 2 - TEXT_OFFSET_X;
    const halfHeight = screenHeight / 2 - TEXT_OFFSET_Y;
    
    function updatePosition() {
        posX += directionX;
        posY += directionY;
        
        if (posX > halfWidth || posX < -halfWidth) {
            directionX *= -1;
            colorIndex = (colorIndex + 1) % colorCycle.length;
            textComponent.SetColor(UE.Color.FromHex(colorCycle[colorIndex]));
        }
        
        if (posY > halfHeight || posY < -halfHeight) {
            directionY *= -1;
            colorIndex = (colorIndex + 1) % colorCycle.length;
            textComponent.SetColor(UE.Color.FromHex(colorCycle[colorIndex]));
        }
        
        textRoot.SetUIRelativeLocation(new UE.Vector(posX, posY, 0));
    }
    
    setInterval(updatePosition, 16);
};

_a.vOo();
