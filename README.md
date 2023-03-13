# spacial_audio
an automation for handling left-right channel mixing and intensity falloff as a function of position


This little project is me sharpening my claws on rust. The use case I have in mind is craft-built game engines, though other applications may emerge. As of 2023/3/12, it's taking x and y coordinates as separate inputs, first computing distance and heading, then using that to calculate left/right channel mix and overall volume adjustments.

todo: Shouldn't be hard to add a third axis, right?
