# logass

#### Description
Log filtering, processing tools.
- Log filtering: remove lines that do not require attention
- Log color: rendered into different colors according to different levels

#### Software Architecture
Software architecture description

#### Installation
cargo build --release

#### Instructions
plugins directory, which saves the corresponding configuration of the plugin. The file name is the same as the plug-in.
For example: color (color filtering) plug-in, the configuration file name is: color.json

#### Contribution

1.  Fork the repository
2.  Create Feat_xxx branch
3.  Commit your code
4.  Create Pull Request


#### Feature
- Plug-in system: can realize row-based filtering, rendering, analysis, statistics, etc.
- Plug-in parameters are configurable