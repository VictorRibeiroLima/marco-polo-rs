-- Add up migration script here
CREATE TYPE video_platforms AS ENUM (
  'YOUTUBE',
  'FACEBOOK',
  'INSTAGRAM',
  'TIKTOK',
  'TWITCH',
  'VIMEO',
  'DAILYMOTION',
  'LINKEDIN',
  'TWITTER',
  'PINTEREST',
  'SNAPCHAT',
  'TIK_TOK',
  'TUMBLR',
  'REDDIT',
  'WHATSAPP',
  'TELEGRAM',
  'VK',
  'OK',
  'WEIBO',
  'WECHAT',
  'LINE',
  'KAKAOTALK'
);

ALTER TABLE
  channels
ADD
  COLUMN platform video_platforms NOT NULL DEFAULT 'YOUTUBE';