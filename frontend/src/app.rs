use zoon::*;

use std::ptr;
use std::sync::Arc;
use web_sys::HtmlAudioElement;

mod duration;
pub mod view;

use duration::Duration;

// -- sound --
fn play_sound(filename: String) {
    Task::start(async move {
        JsFuture::from(
            HtmlAudioElement::new_with_src(&format!("/_api/public/{}", filename))
                .unwrap()
                .play() //todo: add set_preload("auto")
                .unwrap(),
        )
        .await
        .unwrap();
    });
}

fn play_bowl() {
    play_sound("tibetan-bowl.ogg".to_owned());
}

fn play_low_bowl() {
    play_sound("tibetan-bowl-low.ogg".to_owned());
}

fn play_silence_1s() {
    play_sound("silence_1s.wav".to_owned());
}

// -- timer --
#[static_ref]
fn timers() -> &'static MutableVec<Arc<Duration>> {
    let mutable = MutableVec::new_with_values(
        vec![5*60, 15*60]
            .into_iter()
            .map(|seconds| Arc::new(Duration::from_secs(seconds)))
            .collect(),
    );
    Task::start(mutable.signal_vec_cloned().for_each(|change| {
        use futures_signals::signal_vec::VecDiff;
        match change {
            VecDiff::InsertAt { .. } | VecDiff::Move { .. } | VecDiff::Push { .. } => {}
            VecDiff::Replace { values } => {
                if !values.iter().any(|v| is_running_timer(v)) {
                    running_timer().take();
                    clk().take();
                }
            }
            VecDiff::UpdateAt { .. } | VecDiff::RemoveAt { .. } | VecDiff::Pop {} => {
                if !timers().lock_ref().iter().any(|v| is_running_timer(v)) {
                    running_timer().take();
                    clk().take();
                }
            }
            VecDiff::Clear {} => {
                running_timer().take();
                clk().take();
            }
        }
        async {}
    }));
    mutable
}

#[static_ref]
fn clk() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
}

#[static_ref]
fn running_timer() -> &'static Mutable<Option<Arc<Duration>>> {
    let (mutable, signal) = Mutable::new_and_signal_cloned(None);
    Task::start(signal.for_each(|timer: Option<Arc<Duration>>| {
        if timer.is_none() {
            clk().set(None);
        }
        remaining().set(timer.and_then(|t| Some(*t)));
        async {}
    }));
    mutable
}

#[static_ref]
fn remaining() -> &'static Mutable<Option<Duration>> {
    Mutable::new(None)
}

#[static_ref]
fn new_timer() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

struct TimerPart {
    pub inner: Arc<Duration>,
    pub selected: SelectedPart,
}

fn select_from(inner: Arc<Duration>, selected: SelectedPart) -> TimerPart {
    TimerPart { inner, selected }
}

#[derive(Clone, Copy, PartialEq)]
enum SelectedPart {
    Minutes,
    Seconds,
}

#[static_ref]
fn selected_timer() -> &'static Mutable<Option<TimerPart>> {
    Mutable::new(None)
}

#[static_ref]
fn edited_timer() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

fn save_edited_timer() {
    let timer = selected_timer().take().unwrap();
    let mut new_timer = Duration::from_secs(timer.inner.seconds);
    if let Some(new_part) = edited_timer().take().and_then(|d| d.parse::<u32>().ok()) {
        match &timer.selected {
            SelectedPart::Minutes => new_timer.update_mins_part(new_part),
            SelectedPart::Seconds => new_timer.update_secs_part(new_part),
        }
        let mut timers = timers().lock_mut();
        let idx = timers
            .iter()
            .position(|t| same_as(&Some(timer.inner.clone()), t))
            .unwrap();
        timers.set_cloned(idx, Arc::new(new_timer));
    }
}

fn add_timer() {
    let mut new_timer = new_timer().lock_mut();
    if let Ok(duration) = new_timer.parse::<u32>() {
        timers()
            .lock_mut()
            .push_cloned(Arc::new(Duration::from_secs(duration)));
        new_timer.clear();
    } else {
        console::log("Parsing failed");
    }
}

fn remove_timer(duration: Arc<Duration>) {
    let mut timers = timers().lock_mut();
    let duration = Some(duration);
    let idx = timers
        .iter()
        .position(move |t| same_as(&duration, t))
        .unwrap();
    timers.remove(idx);
}

fn clk_running() -> impl Signal<Item = bool> {
    clk().signal_ref(Option::is_some)
}

fn start_clk() {
    running_timer().set(timers().lock_mut().first().cloned());
    clk().set(Some(Timer::new(1_000, tick)));
    play_bowl();
}

fn tick() {
    play_silence_1s(); // Keep running in background
    match remaining().get() {
        Some(rem) if rem.seconds <= 1 => {
            let next_timer = get_next_timer();
            if next_timer.is_none() {
                play_low_bowl();
            } else {
                play_bowl();
            }
            running_timer().set(next_timer);
        }
        Some(rem) => remaining().set(Some(Duration::from_secs(rem.seconds - 1))),
        None => console::error("Tick without current_timer"),
    };
}

fn is_selected_timer(timer: TimerPart) -> impl Signal<Item = bool> {
    selected_timer().signal_ref(move |t| match t {
        Some(t) => {
            same_as(&Some(t.inner.clone()), &timer.inner.clone()) && t.selected == timer.selected
        }
        None => false,
    })
}

fn is_running_timer(timer: &Arc<Duration>) -> bool {
    same_as(&running_timer().get_cloned(), timer)
}

fn same_as(reference: &Option<Arc<Duration>>, other: &Arc<Duration>) -> bool {
    //todo: Remove Option
    reference
        .as_ref()
        .map_or(false, |r| ptr::eq(Arc::as_ptr(&r), Arc::as_ptr(other)))
}

fn get_next_timer() -> Option<Arc<Duration>> {
    let timers_lock = timers().lock_ref(); // hold lock in function scope
    let mut timers_iter = timers_lock.iter();
    while let Some(timer) = timers_iter.next() {
        if is_running_timer(timer) {
            return timers_iter.next().cloned();
        }
    }
    console::error("Current timer not found in timers");
    None
}

fn stop_clk() {
    clk().take();
}
