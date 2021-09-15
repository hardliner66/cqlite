mod build;
mod compile;
mod plan;

pub(crate) use plan::{Filter, LoadProperty, MatchStep, NamedEntity, QueryPlan, UpdateStep};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast;
    use crate::runtime::Instruction;
    use plan::*;

    #[test]
    fn build_a_to_b() {
        // (a) -> (b)
        let query = ast::Query {
            match_clauses: vec![ast::MatchClause {
                start: ast::Node::with_annotation(ast::Annotation::with_name("a")),
                edges: vec![(
                    ast::Edge::right(ast::Annotation::empty()),
                    ast::Node::with_annotation(ast::Annotation::with_name("b")),
                )],
            }],
            where_clauses: vec![],
            set_clauses: vec![],
            return_clause: vec!["a", "b"],
        };

        let plan = QueryPlan {
            steps: vec![
                MatchStep::LoadAnyNode { name: 0 },
                MatchStep::LoadOriginEdge { name: 1, node: 0 },
                MatchStep::LoadTargetNode { name: 2, edge: 1 },
            ],
            updates: vec![],
            returns: vec![NamedEntity::Node(0), NamedEntity::Node(2)],
        };

        assert_eq!(plan, QueryPlan::new(&query).unwrap());
    }

    #[test]
    fn compile_a_to_b() {
        let plan = QueryPlan {
            steps: vec![
                MatchStep::LoadAnyNode { name: 0 },
                MatchStep::LoadOriginEdge { name: 1, node: 0 },
                MatchStep::LoadTargetNode { name: 2, edge: 1 },
            ],
            updates: vec![],
            returns: vec![NamedEntity::Node(0), NamedEntity::Node(2)],
        };

        let code = {
            use Instruction::*;
            vec![
                IterNodes,
                LoadNextNode { jump: 11 },
                IterOriginEdges { node: 0 },
                LoadNextEdge { jump: 9 },
                LoadTargetNode { edge: 0 },
                Yield,
                PopNode,
                PopEdge,
                Jump { jump: 3 },
                PopNode,
                Jump { jump: 1 },
                Halt,
            ]
        };

        assert_eq!(code, plan.compile().unwrap().instructions);
    }
}
