# rosu_native

packaged rosu-pp into native library that can use in other language like java

current state: WIP(only one simple function)

input format:
```rust
#[repr(C)]
pub struct OsuMap {
    /// .osu file path not the map folder
    path: *const c_char,
    /// mods combine 
    /// <see>https://docs.rs/rosu-pp/latest/rosu_pp/trait.Mods.html<see/>
    mods: u32,
    /// accuracy
    acc: f64,
    /// miss count
    miss: u32,
    /// scores max combo
    combo: u32,
    /// map max combo
    max_combo: u32,
}
```
output format:

```rust
#[repr(C)]
pub struct PPResult {
    /// pp
    pub pp: f64,
    /// The accuracy portion of the final pp.
    pub pp_acc: f64,
    /// The aim portion of the final pp.
    pub pp_aim: f64,
    /// The speed portion of the final pp.
    pub pp_speed: f64,
    /// Max pp
    pub max_pp: f64,
    /// pp if fc
    pub pp_fc: f64,
    /// map star
    pub map_star: f64,
    /// debug text
    pub debug_text: *const c_char,
}
```

Then you can use in java

example:
```java
public interface Rosu_PP extends StdCallLibrary {
    File dllFile = new File("path/to/native/dll complied file");
    
    Rosu_PP INSTANCE = (Rosu_PP) Native.load(dllFile.getAbsolutePath(),
            Rosu_PP.class);
            
    PPResult cal_pp(OsuMap map);

}
@Structure.FieldOrder({"path","mods","acc","miss","combo","max_combo"})
public class OsuMap extends Structure {

    public String
    path;

    public long
    mods;
    public double
    acc;
    public long
    miss;
    public long
    combo;
    public long
    max_combo;
    //ingored constructor
}
public static void main(String[] args) {
    File osuFile = new File("path/to/osu/file");
    OsuMap map = new OsuMap(
        osuFile.getAbsolutePath(),//path
        0,//mods (None)
        93.89,//acc
        23,//miss
        372,//combo
        2063//max combo
    );
    PPResult ppResult = Rosu_PP.INSTANCE.cal_pp(map);
}
```