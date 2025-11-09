-- SQLite Migration - DOWN
-- This migration drops the images, tags, and image_tags tables

DROP TABLE IF EXISTS image_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS images;

