Represents a Instruction unit for a [`Proc`].

The ProcInst is chained to other ProcInsts to be executed in order by one of the three Proc instances of the game.

They act as a high-level virtual machine to execute [instructions](crate::proc::ProcDesc) in sequence. Said instructions are queued in a ProcDesc array.

A lot of classes inherit from it, so implement [`IsProcInst`] on them to represent this relation.

If it is not possible for some reason, use the [cast](ProcInst::cast) methods to represent the chains as a ProcInst derivate of your choice.

Example:

```
pub fn createmenubind_hook(proc: &mut Il2CppObject<ProcInst>) {
    let casted_child = proc.child.cast_mut::<BasicMenu>();
}
```

Keep in mind that [`IsProcInst`] is much more preferable, and [cast](ProcInst::cast) should only be used as a last resort.
