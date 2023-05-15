use std::collections::VecDeque;

use leptos::*;
use leptos_icons::{FaIcon, Icon};

#[derive(Copy, Clone, Debug)]
pub struct AlertManager {
    pub alerts: RwSignal<VecDeque<Alert>>,
}

impl AlertManager {
    pub fn new(cx: Scope) -> AlertManager {
        Self {
            alerts: create_rw_signal(cx, VecDeque::new()),
        }
    }

    pub fn add_alert(self, alert: Alert, duration: std::time::Duration) {
        self.alerts.update(|alerts| alerts.push_back(alert));
        set_timeout(move || self.alerts.update(|alerts| { alerts.pop_front(); }), duration);
    }
}

#[component]
pub fn Alerts(cx: Scope, alert_manager: AlertManager) -> impl IntoView {
    let rendered_alerts = move || {
        let alerts = alert_manager
            .alerts
            .get()
            .into_iter()
            .map(|alert| alert.render(cx))
            .collect::<Vec<_>>();
        alerts
    };

    view! { cx,
        <div class="fixed top-32 right-6 flex flex-col gap-2">
            {rendered_alerts}
        </div>
    }
}

#[derive(Clone, Debug)]
pub struct Alert {
    pub text: String,
}

impl Alert {
    fn render(&self, cx: Scope) -> impl IntoView {
        view! { cx,
            <div class="flex w-72 h-14 rounded-md bg-green-300 dark:bg-green-600 px-4 gap-2 justify-between items-center">
                <p class="line-clamp-1 break-all">{&self.text}</p>
                <Icon icon=FaIcon::FaCircleCheckSolid width="1.5em" height="1.5em" />
            </div>
        }
    }
}
