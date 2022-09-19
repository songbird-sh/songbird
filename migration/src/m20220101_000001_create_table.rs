use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Artist::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Artist::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Artist::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Album::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Album::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Album::Title).string().not_null())
                    .col(ColumnDef::new(Album::Artist).string().not_null())
                    .col(ColumnDef::new(Album::ArtistId).string())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("album_artist_id_fkey")
                            .from_tbl(Album::Table)
                            .from_col(Album::ArtistId)
                            .to_tbl(Artist::Table)
                            .to_col(Artist::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Track::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Track::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Track::Title).string().not_null())
                    .col(ColumnDef::new(Track::Artist).string().not_null())
                    .col(ColumnDef::new(Track::Album).string().not_null())
                    .col(ColumnDef::new(Track::Genre).string().not_null())
                    .col(ColumnDef::new(Track::Year).integer())
                    .col(ColumnDef::new(Track::Track).integer())
                    .col(ColumnDef::new(Track::Bitrate).integer())
                    .col(ColumnDef::new(Track::SampleRate).integer())
                    .col(ColumnDef::new(Track::BitDepth).integer())
                    .col(ColumnDef::new(Track::Channels).integer())
                    .col(ColumnDef::new(Track::Duration).float())
                    .col(ColumnDef::new(Track::Uri).string())
                    .col(ColumnDef::new(Track::AlbumId).string())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("track_album_id_fkey")
                            .from_tbl(Track::Table)
                            .from_col(Track::AlbumId)
                            .to_tbl(Album::Table)
                            .to_col(Album::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Playlist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Playlist::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Playlist::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Addon::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Addon::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Addon::Name).string().not_null())
                    .col(ColumnDef::new(Addon::Description).string().not_null())
                    .col(ColumnDef::new(Addon::Version).string().not_null())
                    .col(ColumnDef::new(Addon::Author).string().not_null())
                    .col(ColumnDef::new(Addon::Url).string().not_null())
                    .col(ColumnDef::new(Addon::Enabled).boolean().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PlaylistTrack::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlaylistTrack::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlaylistTrack::PlaylistId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlaylistTrack::TrackId).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("playlist_track_playlist_id_fkey")
                            .from_tbl(PlaylistTrack::Table)
                            .from_col(PlaylistTrack::PlaylistId)
                            .to_tbl(Playlist::Table)
                            .to_col(Playlist::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("playlist_track_track_id_fkey")
                            .from_tbl(PlaylistTrack::Table)
                            .from_col(PlaylistTrack::TrackId)
                            .to_tbl(Track::Table)
                            .to_col(Track::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Artist::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Album::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Track::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Playlist::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Addon::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(PlaylistTrack::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Album {
    Table,
    Id,
    Title,
    Artist,
    ArtistId,
}

#[derive(Iden)]
enum Artist {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Track {
    Table,
    Id,
    Title,
    Artist,
    Album,
    Genre,
    Year,
    Track,
    Bitrate,
    SampleRate,
    BitDepth,
    Channels,
    Duration,
    Uri,
    AlbumId,
}

#[derive(Iden)]
enum Playlist {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Addon {
    Table,
    Id,
    Name,
    Description,
    Version,
    Author,
    Url,
    Enabled,
}

#[derive(Iden)]
enum PlaylistTrack {
    Table,
    Id,
    PlaylistId,
    TrackId,
}
