# Phosphorus layouts
layout system for all phosphorus ports (would need replacement of osl layout system or something)

```
#tabs (summit):
    area: left, top, left + @width, bottom,

#view:
    area: left + #tabs@width, top - #info-bar@height right bottom

// the thing on the top of the summit layout
#info-bar (summit):
    area: left + #tabs@width, top, right, top - @height
```