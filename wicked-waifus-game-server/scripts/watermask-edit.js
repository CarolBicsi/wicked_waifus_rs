const UE = require("ue");
const Info_1 = require("../../../Core/Common/Info");
const MathUtils_1 = require("../../../Core/Utils/MathUtils");
const UiLayerType_1 = require("../../Ui/Define/UiLayerType");
const UiLayer_1 = require("../../Ui/UiLayer");

var _a = require('../Module/WaterMask/WaterMaskController').WaterMaskView;

_a.LOo = 0.18;
_a.yOo = 700;
_a.IOo = 700;

let colorCycle = ["#0000FF", "#0044FF", "#0088FF", "#00BBFF", "#00FFFF", "#88FFFF", "#CCFFFF"];
let movementData = []; // Separate array for movement logic
let colorIndex = 0;

_a.vOo = function () {
    void 0 !== _a.SOo && _a.EOo();
    var e = UiLayer_1.UiLayer.GetLayerRootUiItem(UiLayerType_1.ELayerType.WaterMask);
    var t = (_a.SOo = UE.KuroActorManager.SpawnActor(Info_1.Info.World, UE.UIContainerActor.StaticClass(), MathUtils_1.MathUtils.DefaultTransform, void 0), _a.SOo.RootComponent);
    var e = (t.SetDisplayName("WaterMaskContainer"), UE.KuroStaticLibrary.SetActorPermanent(_a.SOo, !0, !0), _a.SOo.K2_AttachRootComponentTo(e), t.GetRootCanvas().GetOwner().RootComponent);
    var i = e.widget.width % _a.yOo / 2;
    var r = e.widget.height % _a.IOo / 2;
    var n = e.widget.width / 2;
    var _ = e.widget.height / 2;
    var s = Math.ceil(e.widget.width / _a.yOo);
    var o = Math.ceil(e.widget.height / _a.IOo);
    var v = " ";
    
    let textActors = [];
    
    for (let a = 0; a < s; a++) {
        for (let e = 0; e < o; e++) {
            var E = UE.KuroActorManager.SpawnActor(Info_1.Info.World, UE.UITextActor.StaticClass(), MathUtils_1.MathUtils.DefaultTransform, void 0);
            var U = E.RootComponent;
            var U = (E.K2_AttachRootComponentTo(t), U.SetDisplayName("WaterMaskText"), E.GetComponentByClass(UE.UIText.StaticClass()));
            
            U.SetFontSize(_a.vFt);
            U.SetOverflowType(0);
            U.SetAlpha(_a.LOo);
            U.SetFont(UE.LGUIFontData.GetDefaultFont());
            U.SetText(v);
            
            let basePosition = new UE.Vector(a * _a.yOo - n + i, e * _a.IOo - _ + r, 0);
            U.SetUIRelativeLocation(basePosition);
            
            textActors.push(U);
            movementData.push(basePosition); // Separate array for position tracking
        }
    }
    
    function animateCrossMovement() {
        let time = Date.now() * 0.002;
        
        textActors.forEach((U, index) => {
            let basePosition = movementData[index];
            let offsetX = Math.sin(time + index * 0.5) * 50;
            let offsetY = Math.cos(time + index * 0.5) * 50;
            U.SetUIRelativeLocation(new UE.Vector(basePosition.X + offsetX, basePosition.Y + offsetY, basePosition.Z));
        });
    }
    
    function animateColors() {
        colorIndex = (colorIndex + 1) % colorCycle.length;
        textActors.forEach(U => {
            U.SetColor(UE.Color.FromHex(colorCycle[colorIndex]));
        });
    }
    
    setInterval(animateColors, 1000); // Change color every second
    setInterval(animateCrossMovement, 16); // Update every 16ms (~60 FPS)
};

_a.vOo();