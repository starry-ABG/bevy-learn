# 如何来实现？

核心：要以ECS的思路去思考。

页面上可能有许多的inputbox，点击不同的inputbox需要将焦点切换过去。Interaction可以用于交互，能够获取到鼠标的点击事件。

对于选中的inputbox需要能够改变其内容。对于未选中的inputbox，键盘的输入应该对其没有影响。

inputbox包含：
1. 外形。外形需要有相关属性来确定它的大小，起可能来自于Node。从Button的实现来看，其体现了这一点。Button中的功能依赖了Node，Interaction和focus policy。所以Node可以为entity确定大小。interaction可以为其加入交互的功能。理解node是一个关键。另一方面，考虑到button已经实现了交互。所以也可以直接在其基础上来实现。
2. 内容。可以用Text加TextSpan来实现。
    a. 输入内容
    b. 输入法内容
    c. 光标

## 问题

1. 如何将输入法的框放在合适的地方？

2. 如何支持多个inputbox？

