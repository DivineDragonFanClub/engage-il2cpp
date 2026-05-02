Represents an instruction unit for a [`Proc`](crate::app::proc::Proc).

The ProcInst is chained to other ProcInsts to be executed in order by one of the three Proc instances of the game.

They act as a high-level virtual machine to execute [instructions](crate::app::procdesc::ProcDesc) in sequence. Said instructions are queued in a ProcDesc array.

A lot of classes inherit from it; the bindgen emits `#[parent(...)]` chains so derived types coerce to `ProcInst` via the standard `Into` impls. For type-erased downcasts, use the methods on [`unity2::Cast`].
