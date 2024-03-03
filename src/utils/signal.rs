use leptos::*;

pub fn create_initial_signal<T>(initial : impl SignalGet<Value = T> + SignalGetUntracked<Value = T> + 'static) -> (ReadSignal<T>, WriteSignal<T>) where T:Clone {
    let (getter, setter) = create_signal(initial.get_untracked());
    
    create_effect(move |_| {
      setter.set(initial.get());
    });
    (getter, setter)
}

pub fn create_initial_rw_signal<T>(initial : impl SignalGet<Value = T> + SignalGetUntracked<Value = T> + 'static) -> RwSignal<T> where T:Clone {
    let signal = create_rw_signal(initial.get_untracked());
    
    create_effect(move |_| {
        signal.set(initial.get());
    });
    signal
}
