# Checks whether any node has a col hierarchy suffix. If it does, the entire
# subtree will be converted to static collision bodies.

@tool
extends EditorScenePostImport

func _post_import(scene):
	find_hierarchy_root(scene, scene)
	return scene

static var PREFIX: String = "_colhierarchy"

# Recursive function to find and call process_hierarchy_root on any nodes
# containign the suffix.
func find_hierarchy_root(node, root):
	if node == null:
		return

	if node.name.ends_with(PREFIX):
		process_hierarchy_root(node, root)
		return

	for child in node.get_children():
		find_hierarchy_root(child, root)

# A root has been found, convert all children.
func process_hierarchy_root(node, root):
	if node == null:
		return

	if node is MeshInstance3D:
		var static_body := StaticBody3D.new()
		static_body.transform = node.transform
		#static_body.owner = root
		
		node.replace_by(static_body)
		
		var collision_shape := CollisionShape3D.new()
		collision_shape.shape = node.mesh.create_convex_shape(false, false)

		static_body.add_child(collision_shape, false)
		collision_shape.owner = root

		node.queue_free()
		node = static_body

	for child in node.get_children():
		process_hierarchy_root(child, root)
	pass
