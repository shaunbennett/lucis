## Performance Ideas
- Lazy load normals (prevents calculating normal until we know the closest intersect)
- store hierarchy in a kd-tree
    - need to research how to handle volumes rather than points
- fork-join for rendering
- SIMD