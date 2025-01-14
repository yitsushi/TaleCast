use crate::config::IndicatifSettings;
use crate::episode::Episode;
use crate::utils;
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::sync::Arc;

#[derive(Debug)]
pub struct DownloadBar {
    bar: Option<ProgressBar>,
    podcast_name: String,
    longest_podcast_name: usize,
    settings: Arc<IndicatifSettings>,
}

impl DownloadBar {
    pub fn new(
        podcast_name: String,
        settings: Arc<IndicatifSettings>,
        mp: &MultiProgress,
        longest_podcast_name: usize,
    ) -> Self {
        let bar = if settings.enabled() {
            let bar = mp.add(ProgressBar::new_spinner());
            Some(bar)
        } else {
            None
        };

        Self {
            bar,
            settings,
            podcast_name,
            longest_podcast_name,
        }
    }

    pub fn init(&self) {
        if let Some(pb) = &self.bar {
            let template = self.settings.download_template();
            pb.set_style(ProgressStyle::default_bar().template(&template).unwrap());
            pb.enable_steady_tick(self.settings.spinner_speed());
        }
    }

    pub fn begin_download(&self, episode: &Episode, index: usize, episode_qty: usize) {
        if let Some(pb) = &self.bar {
            let fitted_episode_title = {
                let title_length = self.settings.title_length();
                let padded = &format!("{:<width$}", episode.title, width = title_length);
                utils::truncate_string(padded, title_length, true)
            };

            let msg = format!(
                "{:<podcast_width$} {}/{} {} ",
                &self.podcast_name,
                index + 1,
                episode_qty,
                &fitted_episode_title,
                podcast_width = self.longest_podcast_name + 3
            );

            pb.set_message(msg);
            pb.set_position(0);
        }
    }

    pub fn set_template(&self, style: &str) {
        if let Some(pb) = &self.bar {
            pb.set_style(ProgressStyle::default_bar().template(style).unwrap());
        }
    }

    pub fn hook_status(&self) {
        let template = self.settings.hook_template();
        self.set_template(&template);
    }

    pub fn init_download_bar(&self, start_point: u64, total_size: u64) {
        if let Some(pb) = &self.bar {
            pb.set_length(total_size);
            pb.set_position(start_point);
        }
    }

    pub fn set_progress(&self, progress: u64) {
        if let Some(pb) = &self.bar {
            pb.set_position(progress);
        }
    }

    pub fn complete(&self) {
        if let Some(pb) = &self.bar {
            let template = self.settings.completion_template();
            self.set_template(&template);
            pb.finish_with_message(self.podcast_name.clone());
        }
    }
}
