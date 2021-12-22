struct AABB
{
    in_x: (i32, i32),
    in_y: (i32, i32)
}

pub fn day_17_1() -> i32
{
    let target = AABB{ in_x: (209, 238), in_y: (59, -86)};
    return (target.in_y.1 * (target.in_y.1 + 1)) / 2;
}
