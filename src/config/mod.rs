use std::{fs, io};
use std::io::Write;
use exitfailure::ExitFailure;
use failure::err_msg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub music_database: String,
    pub theme: InitTheme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitTheme {
    pub list_title_color: String,
    pub list_title_page_color: String,
    pub list_border_color: String,
    pub list_music_color: String,
    pub list_folder_color: String,
    pub list_icon_color: String,
    pub list_selected_color: String,
    pub search_border_color: String,
    pub search_icon_color: String,
    pub search_font_color: String,
    pub command_font_color: String,
    pub command_border_color: String,
    pub music_pic_color1: String,
    pub music_pic_color2: String,
    pub usage_color_left: String,
    pub usage_color_right: String,
    pub cut_off_rule_color: String,
    pub play_music_list_title_color: String,
    pub play_music_list_border_color: String,
    pub play_music_list_id_color: String,
    pub play_music_list_duration_color: String,
    pub play_music_list_name_color: String,
    pub play_music_list_artist_color: String,
    pub play_music_list_album_color: String,
    pub play_music_list_header_color: String,
    pub playing_music_border_color: String,
    pub playing_music_name_color: String,
    pub volume_icon_color: String,
    pub volume_value_color: String,
    pub gauge_color: String,
    pub gauge_border_color: String,
    pub gauge_label_color: String,
}

fn prompt_for_music_database_path() -> Result<String, ExitFailure> {
    println!("Please enter the path for the music folder");

    // 等待用户输入音乐数据库路径
    let mut input = String::new();
    io::stdout().flush()?;  // 刷新输出缓冲区以确保提示语显示

    io::stdin().read_line(&mut input)?;
    let input = input.trim();  // 去掉输入中的多余空格和换行符

    if input.is_empty() {
        return Err(ExitFailure::from(err_msg("Music database path cannot be empty")));
    }

    Ok(input.to_string())
}

pub fn init() -> Result<InitConfig, ExitFailure> {
    let config_path = dirs::home_dir()
        .map(|home| home.join(".config").join("music_player").join("config.yml"))
        .ok_or_else(|| ExitFailure::from(err_msg("Home directory not found")))?;

    let config_dir = config_path.parent().unwrap(); // 获取目录路径
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).map_err(|_e| ExitFailure::from(err_msg("Failed to create config directory")))?;
    }

    if !config_path.exists() {
        // 配置文件不存在，创建默认配置
        let music_database = prompt_for_music_database_path()?;

        let default_config = InitConfig {
            music_database,
            theme: InitTheme {
                list_title_color: String::from("#ffaaff"),
                list_title_page_color: String::from("#ffb747"),
                list_border_color: String::from("#ffffff"),
                list_music_color: String::from("#eee4c4"),
                list_folder_color: String::from("#eee4c4"),
                list_icon_color: String::from("#f07178"),
                list_selected_color: String::from("#c3e88d"),
                search_border_color: String::from("#ffb747"),
                search_icon_color: String::from("#ec998b"),
                search_font_color: String::from("#eee4c4"),
                command_font_color: String::from("#eee4c4"),
                command_border_color: String::from("#c3eead"),
                music_pic_color1: String::from("#f07178"),
                music_pic_color2: String::from("#81a8fd"),
                usage_color_left: String::from("#beb2ec"),
                usage_color_right: String::from("#eee188"),
                cut_off_rule_color: String::from("#c3e88d"),
                play_music_list_title_color: String::from("#81a8fd"),
                play_music_list_border_color: String::from("#ffaaff"),
                play_music_list_id_color: String::from("#e0d7ca"),
                play_music_list_duration_color: String::from("#a9c34f"),
                play_music_list_name_color: String::from("#eee4c4"),
                play_music_list_artist_color: String::from("#b2e2e4"),
                play_music_list_album_color: String::from("#eee188"),
                play_music_list_header_color: String::from("#d15aa7"),
                playing_music_border_color: String::from("#81a8fd"),
                playing_music_name_color: String::from("#d8ce2e"),
                volume_icon_color: String::from("#9998af"),
                volume_value_color: String::from("#dcd8da"),
                gauge_color: String::from("#cece68"),
                gauge_border_color: String::from("#abcc7e"),
                gauge_label_color: String::from("#fa4d70"),
            },
        };

        let file = fs::File::create(&config_path)
            .map_err(|_| ExitFailure::from(err_msg("Failed to create config file")))?;

        serde_yaml::to_writer(file, &default_config)
            .map_err(|_e| ExitFailure::from(err_msg("Failed to write default config")))?;

        println!("Default config file created at {:?}", config_path);
        return Ok(default_config);
    }

    let file = fs::File::open(config_path)
        .map_err(|_| ExitFailure::from(err_msg("Failed to open config file")))?;

    let init_config: InitConfig = serde_yaml::from_reader(file)
        .map_err(|_e| ExitFailure::from(err_msg("Failed to parse config file:")))?;

    Ok(init_config)
}
