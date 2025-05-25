setTimeout(() => {
    const UiManager_1 = require("../Ui/UiManager");
    const UE = require("ue");
    const ControllerManagerBase_1 = require("../../Core/Framework/ControllerManagerBase");
    
    const UiText = UiManager_1.UiManager.GetViewByName("UidView").GetText(0);
    UiText.SetText("{PLAYER_USERNAME} - WuWa Start");
    
    const colorCycle = [
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
    
    // 移除原来的静态颜色设置:
    // UiText.SetColor(UE.Color.FromHex("{SELECTED_COLOR}"));
    
    setInterval(() => {
        colorIndex = (colorIndex + 1) % colorCycle.length;
        UiText.SetColor(UE.Color.FromHex(colorCycle[colorIndex]));
    }, 1000); // 每1000毫秒（1秒）改变一次颜色，您可以根据需要调整时间间隔
}, 10000);

