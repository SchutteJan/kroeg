/* eslint-disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type AreaTypeEnum = 'Neighbourhood' | 'District' | 'Area' | 'Borough'
export type Email = string
export type UserRoleEnum = 'Admin' | 'User'

export interface ExportedSchemas {
	_area: Area
	_location_response: LocationResponse
	/**
	 * @minItems 3
	 * @maxItems 3
	 */
	_user: [Login, WhoResponse, VisitStats]
}
export interface Area {
	area: PolygonFor_Point
	area_type: AreaTypeEnum
	id: number
	name: string
}
/**
 * Use that structure in `Insertable` or `Queryable` struct if you work with Polygon geometry. ``` #[macro_use] extern crate diesel; use postgis_diesel::types::{Polygon,Point}; #[derive(Queryable)] struct QueryablePolygonExample { id: i32, polygon: Polygon<Point>, } ```
 */
export interface PolygonFor_Point {
	rings: Point[][]
	srid?: number | null
}
/**
 * Use that structure in `Insertable` or `Queryable` struct if you work with Point geometry. ``` #[macro_use] extern crate diesel; use postgis_diesel::types::Point; #[derive(Queryable)] struct QueryablePointExample { id: i32, point: Point, } ```
 */
export interface Point {
	srid?: number | null
	x: number
	y: number
}
export interface LocationResponse {
	address_line: string
	area_name?: string | null
	coordinates: Point
	description?: string | null
	gem_ams_id?: number | null
	google_place_id?: string | null
	id: number
	imageurl?: string | null
	name: string
	visited_at?: string | null
}
export interface Login {
	email: Email
	password: string
}
export interface WhoResponse {
	id: number
	role: UserRoleEnum
}
export interface VisitStats {
	bar_visits_by_area: [string, number][]
	distinct_bar_visits: number
	total_bars_by_area: [string, number][]
}
