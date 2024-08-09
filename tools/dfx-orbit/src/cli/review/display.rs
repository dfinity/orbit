use orbit_station_api::{ListRequestsResponse, RequestStatusDTO};
use tabled::{
    settings::{Settings, Style},
    Table,
};

pub(crate) fn display_list(data: &ListRequestsResponse) {
    let data_iter = data.requests.iter().map(|request| {
        [
            request.id.clone(),
            request.title.clone(),
            request.summary.clone().unwrap_or(String::from("-")),
            match request.status {
                RequestStatusDTO::Created => String::from("Created"),
                RequestStatusDTO::Approved => String::from("Approved"),
                RequestStatusDTO::Rejected => String::from("Rejected"),
                RequestStatusDTO::Cancelled { .. } => String::from("Cancelled"),
                RequestStatusDTO::Scheduled { .. } => String::from("Scheduled"),
                RequestStatusDTO::Processing { .. } => String::from("Processing"),
                RequestStatusDTO::Completed { .. } => String::from("Completed"),
                RequestStatusDTO::Failed { .. } => String::from("Failed"),
            },
        ]
    });
    let titled_iter = std::iter::once([
        String::from("ID"),
        String::from("Title"),
        String::from("Summary"),
        String::from("Status"),
    ])
    .chain(data_iter);

    let table_config = Settings::default().with(Style::psql());
    let table = Table::from_iter(titled_iter).with(table_config).to_string();
    println!("{}", table);
}
