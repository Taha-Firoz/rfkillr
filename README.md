## rfkillr 🦀🔪 

Send `rfkill` commands through rust, not `async`

example
```
   let rfkill = RfKill::new()?;

    let reset_event = CRfKillEvent::default()
    .set_event_type(RfkillType::All)
    .set_op(RfkillOperations::RfKillOpChangeAll)
    .soft_block();

    rfkill.update_device(reset_event);
```