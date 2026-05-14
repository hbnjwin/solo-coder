package com.linkyoyo.tobacco.dto;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.ArrayList;
import java.util.List;

/**
 * 标准图谱数据传输对象
 * 用于前端可视化展示
 */
@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class StandardGraphDto {
    
    /**
     * 节点列表
     */
    private List<Node> nodes = new ArrayList<>();
    
    /**
     * 关系列表
     */
    private List<Link> links = new ArrayList<>();
    
    /**
     * 图谱节点
     */
    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    @Builder
    public static class Node {
        /**
         * 节点ID
         */
        private String id;
        
        /**
         * 节点标签
         */
        private String label;
        
        /**
         * 节点类型
         */
        private String type;
        
        /**
         * 节点属性
         */
        private Object properties;
    }
    
    /**
     * 图谱关系
     */
    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    @Builder
    public static class Link {
        /**
         * 关系ID
         */
        private String id;
        
        /**
         * 源节点ID
         */
        private String source;
        
        /**
         * 目标节点ID
         */
        private String target;
        
        /**
         * 关系类型
         */
        private String type;
        
        /**
         * 关系标签
         */
        private String label;
        
        /**
         * 关系属性
         */
        private Object properties;
    }
}
