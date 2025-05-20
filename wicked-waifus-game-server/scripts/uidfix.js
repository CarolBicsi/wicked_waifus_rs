setTimeout(() => {
    const UiManager_1 = require("../Ui/UiManager");
    const UE = require("ue");
    const ControllerManagerBase_1 = require("../../Core/Framework/ControllerManagerBase");
    
    const UiText = UiManager_1.UiManager.GetViewByName("UidView").GetText(0);
    
    // 使用预定义颜色 - 每次运行时随机选择一种颜色方案
    const colorSchemes = [
        // 蓝色渐变
        ["#0000FF", "#0044FF", "#0088FF", "#00BBFF", "#00FFFF", "#88FFFF", "#CCFFFF"],
    ];
    
    // 随机选择一种配色方案
    const selectedScheme = colorSchemes[Math.floor(Math.random() * colorSchemes.length)];
    
    // 设置文本
    UiText.SetText("原神再起不能动");
    
    // 随机选择一个颜色应用到整个文本
    const randomColorIndex = Math.floor(Math.random() * selectedScheme.length);
    const selectedColor = selectedScheme[randomColorIndex];
    
    // 应用颜色
    UiText.SetColor(UE.Color.FromHex(selectedColor));
    
    console.log("应用了颜色：" + selectedColor);
}, 10000);
