using Godot;

public partial class JumpGate : Node3D {
    [Export]
    public Node3D inner_ring;
    [Export]
    public Node3D outer_ring;

    public float inner_velocity = inner_velocity_default;
    public float outer_velocity = 0.0f;

    const float inner_velocity_default = Mathf.Pi / 40;

    public override void _Process(double delta) {
        if (inner_velocity > 0.0f) {
            inner_ring.RotateZ(inner_velocity * (float) delta);
        }

        if (outer_velocity > 0.0f) {
            outer_ring.RotateX(outer_velocity * (float) delta);
        }
    }
}
