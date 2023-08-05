// TODO : Delete me

//pub fn build_list_item_from_podcast(podcast: Podcast, available_width: usize) -> ListItem {
//    let vec_spans = iter::once(Spans::from(Span::styled::<String>(
//        podcast.title.clone(),
//        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
//    )));
//
//    let hash_display = iter::once(Spans::from(vec![
//        Span::from("["),
//        Span::styled(
//            podcast.hash(),
//            Style::default()
//                .add_modifier(Modifier::BOLD)
//                .fg(Color::LightGreen),
//        ),
//        Span::from("]"),
//    ]));
//    let vec_spans = vec_spans.chain(hash_display);
//
//    let description_style = Style::default().add_modifier(Modifier::ITALIC);
//
//    let lines = str_to_lines(&podcast.description, available_width)
//        .into_iter()
//        .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
//    let vec_spans = vec_spans.chain(lines).collect::<Vec<Spans>>();
//
//    ListItem::new(vec_spans)
//}
//
//pub fn build_list_item_from_episode(episode: PodcastEpisode, available_width: usize) -> ListItem {
//    let vec_spans = iter::once(Spans::from(Span::styled(
//        episode.title.clone(),
//        Style::default().bg(Color::LightMagenta).fg(Color::Red),
//    )));
//
//    let metadata_display = iter::once(Spans::from(vec![
//        Span::from("["),
//        Span::styled(
//            episode.hash(),
//            Style::default()
//                .add_modifier(Modifier::BOLD)
//                .fg(Color::LightGreen),
//        ),
//        Span::from("]"),
//        Span::from("   "),
//        Span::styled("Date:", Style::default().add_modifier(Modifier::ITALIC)),
//        Span::from(" "),
//        Span::styled(
//            format!("{}", episode.pub_date.format("%d/%m/%Y")),
//            Style::default()
//                .bg(Color::Black)
//                .add_modifier(Modifier::ITALIC),
//        ),
//    ]));
//    let vec_spans = vec_spans.chain(metadata_display);
//
//    let description_style = Style::default().add_modifier(Modifier::ITALIC);
//
//    let description = episode.description;
//    let lines = str_to_lines(&description, available_width)
//        .into_iter()
//        .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
//    //let vec_spans: Vec<Spans> = vec_spans.chain(lines).collect::<Vec<Spans>>();
//    let vec_spans: Vec<Spans> = vec![Spans::from(vec![])];
//    ListItem::new(Span::raw("mdr"))
//}
//
